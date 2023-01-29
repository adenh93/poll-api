use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[get("/polls/{id}")]
pub async fn get_poll(_id: web::Path<Uuid>, _conn: web::Data<PgPool>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
