use crate::{
    domain::NewPoll,
    helpers::parse_client_ip,
    repositories::{
        create_new_poll_and_choices, get_poll_by_id, get_poll_vote_by_ip_address, insert_poll_vote,
    },
};
use actix_web::{get, post, web, HttpRequest, HttpResponse};
use actix_web_validator::Json;
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(name = "Fetching a poll", skip(conn))]
#[get("/polls/{id}")]
pub async fn get_poll(id: web::Path<Uuid>, conn: web::Data<PgPool>) -> HttpResponse {
    let result = get_poll_by_id(&id, &conn).await;

    if let Ok(poll) = result {
        return HttpResponse::Ok().json(poll);
    }

    match result.err() {
        Some(sqlx::Error::RowNotFound) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
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
) -> HttpResponse {
    let poll = match get_poll_by_id(&path.id, &conn).await {
        Ok(result) => result,
        Err(err) => {
            return match err {
                sqlx::Error::RowNotFound => HttpResponse::NotFound().finish(),
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    };

    if poll.end_date < Utc::now() {
        return HttpResponse::BadRequest().finish();
    }

    if let Ok(ip_address) = parse_client_ip(&req.connection_info().realip_remote_addr()) {
        match get_poll_vote_by_ip_address(&poll.id, &ip_address, &conn).await {
            Ok(Some(_)) => return HttpResponse::BadRequest().finish(),
            Err(_) => return HttpResponse::InternalServerError().finish(),
            _ => (),
        };

        if let Err(err) = insert_poll_vote(&path.id, &path.choice, &ip_address, &conn).await {
            return match err {
                sqlx::Error::Database(_) => HttpResponse::BadRequest().finish(),
                _ => HttpResponse::InternalServerError().finish(),
            };
        }

        return HttpResponse::Ok().finish();
    }

    HttpResponse::InternalServerError().finish()
}

#[tracing::instrument(name = "Creating a poll", skip(conn))]
#[post("/polls")]
pub async fn create_poll(new_poll: Json<NewPoll>, conn: web::Data<PgPool>) -> HttpResponse {
    let result = create_new_poll_and_choices(&new_poll, &Utc::now(), &conn).await;

    match result {
        Ok(receipt) => HttpResponse::Created().json(receipt),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
