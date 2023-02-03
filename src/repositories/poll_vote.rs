use crate::domain::PollVote;
use chrono::Utc;
use sqlx::{types::ipnetwork::IpNetwork, PgPool};
use uuid::Uuid;

#[tracing::instrument(
    name = "Fetch poll vote by IP Address",
    skip(poll_id, ip_address, conn)
)]
pub async fn get_poll_vote_by_ip_address(
    poll_id: &Uuid,
    ip_address: &IpNetwork,
    conn: &PgPool,
) -> sqlx::Result<Option<PollVote>> {
    let vote = sqlx::query_as!(
        PollVote,
        r#"
        SELECT id, poll_id, choice_id, created_at
        FROM poll_vote
        WHERE poll_id = $1 AND ip_address = $2
        "#,
        poll_id,
        ip_address,
    )
    .fetch_optional(conn)
    .await?;

    Ok(vote)
}

#[tracing::instrument(
    name = "Inserting poll vote record",
    skip(poll_id, choice_id, ip_address, conn)
)]
pub async fn insert_poll_vote(
    poll_id: &Uuid,
    choice_id: &Uuid,
    ip_address: &IpNetwork,
    conn: &PgPool,
) -> sqlx::Result<()> {
    let uuid = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO poll_vote (id, poll_id, choice_id, ip_address, created_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        uuid,
        poll_id,
        choice_id,
        ip_address,
        Utc::now(),
    )
    .execute(conn)
    .await?;

    Ok(())
}
