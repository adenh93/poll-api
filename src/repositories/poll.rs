use crate::domain::{created_poll::CreatedPoll, new_poll::NewPoll, new_poll_choice::NewPollChoice};
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Postgres, QueryBuilder, Transaction};
use uuid::Uuid;

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
