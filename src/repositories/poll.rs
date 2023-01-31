use crate::domain::{CreatedPoll, NewPoll, NewPollChoice, Poll, PollChoice, PollResults};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres, QueryBuilder, Transaction};
use uuid::Uuid;

#[tracing::instrument(name = "Getting poll by id", skip(id, conn))]
pub async fn get_poll_by_id(id: &Uuid, conn: &PgPool) -> sqlx::Result<Poll> {
    let result = sqlx::query_as!(
        Poll,
        r#"
         SELECT
           poll.id, poll.name, poll.description, poll.created_at, poll.end_date,
           array_agg((choice.id, choice.name, choice.created_at)) as "choices!: Vec<PollChoice>"
           FROM poll as poll
           LEFT OUTER JOIN poll_choice as choice
           ON choice.poll_id = poll.id
           WHERE poll.id = $1
           GROUP BY poll.id
         "#,
        id
    )
    .fetch_one(conn)
    .await
    .map_err(|err| {
        tracing::error!("Failed to execute query: {err:?}");
        err
    })?;

    Ok(result)
}

#[tracing::instrument(name = "Creating new poll and choices", skip(new_poll, conn))]
pub async fn create_new_poll_and_choices(
    new_poll: &NewPoll,
    start_date: &DateTime<Utc>,
    conn: &PgPool,
) -> sqlx::Result<CreatedPoll> {
    let mut tx = conn.begin().await?;

    let uuid = insert_poll(&new_poll, start_date, &mut tx).await?;
    insert_poll_choices(&uuid, &new_poll.choices, &mut tx).await?;

    tx.commit().await?;

    Ok(CreatedPoll {
        id: uuid,
        name: new_poll.name.to_string(),
    })
}

#[tracing::instrument(name = "Inserting poll record", skip(new_poll, tx))]
pub async fn insert_poll(
    new_poll: &NewPoll,
    start_date: &DateTime<Utc>,
    tx: &mut Transaction<'_, Postgres>,
) -> sqlx::Result<Uuid> {
    let uuid = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO poll (id, name, description, created_at, end_date)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        uuid,
        &new_poll.name,
        new_poll.description,
        &start_date,
        &new_poll.end_date
    )
    .execute(tx)
    .await
    .map_err(|err| {
        tracing::error!("Failed to execute query: {err:?}");
        err
    })?;

    Ok(uuid)
}

#[tracing::instrument(name = "Inserting poll choice records", skip(poll_uuid, choices, tx))]
pub async fn insert_poll_choices(
    poll_uuid: &Uuid,
    choices: &Vec<NewPollChoice>,
    tx: &mut Transaction<'_, Postgres>,
) -> sqlx::Result<()> {
    let mut builder = QueryBuilder::new("INSERT INTO poll_choice (id, name, poll_id, created_at)");

    builder.push_values(choices.iter(), |mut b, choice| {
        b.push_bind(Uuid::new_v4())
            .push_bind(&choice.name)
            .push_bind(&poll_uuid)
            .push_bind(Utc::now());
    });

    builder.build().execute(tx).await.map_err(|err| {
        tracing::error!("Failed to execute query: {err:?}");
        err
    })?;

    Ok(())
}

#[tracing::instrument(name = "Fetching poll results by id", skip(poll_id, conn))]
pub async fn get_poll_results_by_id(
    poll_id: &Uuid,
    conn: &PgPool,
) -> sqlx::Result<Vec<PollResults>> {
    let results = sqlx::query_as!(
        PollResults,
        r#"
        SELECT 
        pc.id as choice_id,
        pc.poll_id,
        (
          SELECT COUNT(pv.id) 
          FROM poll_vote pv
          WHERE pv.choice_id = pc.id
        ) as vote_count
        FROM poll_choice pc
        WHERE pc.poll_id = $1
        GROUP BY pc.poll_id, pc.id
        ORDER BY vote_count DESC
        "#,
        poll_id
    )
    .fetch_all(conn)
    .await
    .map_err(|err| {
        tracing::error!("Failed to execute query: {err:?}");
        err
    })?;

    Ok(results)
}
