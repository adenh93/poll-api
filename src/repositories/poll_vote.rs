use sqlx::{types::ipnetwork::IpNetwork, PgPool};
use uuid::Uuid;

use crate::domain::PollVote;

#[tracing::instrument(
    name = "Fetch poll vote by IP Address",
    skip(poll_uuid, ip_address, conn)
)]
pub async fn get_poll_vote_by_ip_address(
    poll_uuid: &Uuid,
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
        poll_uuid,
        ip_address,
    )
    .fetch_optional(conn)
    .await
    .map_err(|err| {
        tracing::error!("Failed to execute query: {err:?}");
        err
    })?;

    Ok(vote)
}
