use crate::{
    domain::NewPoll,
    errors::{HttpError, HttpResult},
    helpers::parse_client_ip,
    repositories::{
        poll::{create_new_poll_and_choices, get_poll_by_id, get_poll_results_by_id},
        poll_vote::{get_poll_vote_by_ip_address, insert_poll_vote},
    },
};
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use anyhow::Context;
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

#[tracing::instrument(name = "Fetching a poll", skip(conn))]
#[get("/polls/{id}")]
pub async fn get_poll(id: web::Path<Uuid>, conn: web::Data<PgPool>) -> HttpResult<HttpResponse> {
    let poll = get_poll_by_id(&id, &conn)
        .await
        .context("Failed to retrieve poll from database.")?;

    if poll.is_none() {
        return Err(HttpError::NotFoundError(
            "The requested poll could not be found.".into(),
        ));
    }

    Ok(HttpResponse::Ok().json(poll))
}

#[tracing::instrument(name = "Fetching poll results", skip(conn))]
#[get("/polls/{id}/results")]
pub async fn get_poll_results(
    id: web::Path<Uuid>,
    conn: web::Data<PgPool>,
) -> HttpResult<HttpResponse> {
    let poll = get_poll_by_id(&id, &conn)
        .await
        .context("Failed to retrieve poll from database.")?;

    if let Some(poll) = poll {
        if poll.end_date > Utc::now() {
            return Err(HttpError::UserError(
                "Cannot retrieve results for an active poll.",
            ));
        }

        let results = get_poll_results_by_id(&id, &conn)
            .await
            .context("Failed to retrieve poll results from database.")?;

        Ok(HttpResponse::Ok().json(results))
    } else {
        Err(HttpError::NotFoundError(
            "The requested poll could not be found.",
        ))
    }
}

#[derive(Debug, Deserialize)]
pub struct VotePath {
    pub id: Uuid,
    pub choice: Uuid,
}

#[tracing::instrument(name = "Registering poll vote", skip(conn, req))]
#[post("/polls/{id}/vote/{choice}")]
pub async fn vote_poll(
    path: web::Path<VotePath>,
    conn: web::Data<PgPool>,
    req: HttpRequest,
) -> HttpResult<HttpResponse> {
    let poll = get_poll_by_id(&path.id, &conn)
        .await
        .context("Failed to retrieve poll to cast vote.")?;

    if let Some(poll) = poll {
        if poll.end_date < Utc::now() {
            return Err(HttpError::UserError("Cannot vote in an expired poll."));
        }

        let existing_choice = poll.choices.iter().find(|choice| choice.id == path.choice);

        if existing_choice.is_none() {
            return Err(HttpError::UserError(
                "Cannot cast a vote for an invalid choice",
            ));
        }

        let ip_address = parse_client_ip(&req.connection_info().realip_remote_addr())
            .context("Failed to parse client IP address.")?;

        let poll_vote = get_poll_vote_by_ip_address(&poll.id, &ip_address, &conn)
            .await
            .context("Failed to retrieve existing vote from database")?;

        if poll_vote.is_some() {
            return Err(HttpError::UserError("You have already voted in this poll."));
        }

        insert_poll_vote(&path.id, &path.choice, &ip_address, &conn)
            .await
            .context("Failed to insert new vote into the database.")?;

        Ok(HttpResponse::Ok().finish())
    } else {
        Err(HttpError::NotFoundError(
            "The requested poll could not be found.",
        ))
    }
}

#[tracing::instrument(name = "Creating a poll", skip(conn))]
#[post("/polls")]
pub async fn create_poll(
    new_poll: web::Json<NewPoll>,
    conn: web::Data<PgPool>,
) -> HttpResult<HttpResponse> {
    new_poll.validate().map_err(HttpError::ValidationError)?;
    let receipt = create_new_poll_and_choices(&new_poll, &Utc::now(), &conn).await?;
    Ok(HttpResponse::Created().json(receipt))
}
