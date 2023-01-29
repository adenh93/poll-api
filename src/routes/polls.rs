use actix_web::{get, post, web, HttpResponse};
use actix_web_validator::Json;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    domain::NewPoll,
    repositories::{create_new_poll_and_choices, get_poll_by_id},
};

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

#[tracing::instrument(name = "Creating a poll", skip(conn))]
#[post("/polls")]
pub async fn create_poll(new_poll: Json<NewPoll>, conn: web::Data<PgPool>) -> HttpResponse {
    let result = create_new_poll_and_choices(&new_poll, &Utc::now(), &conn).await;

    match result {
        Ok(receipt) => HttpResponse::Created().json(receipt),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
