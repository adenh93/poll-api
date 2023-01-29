use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(name = "Fetching a poll", skip(conn))]
#[get("/polls/{id}")]
pub async fn get_poll(id: web::Path<Uuid>, conn: web::Data<PgPool>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
