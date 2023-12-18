use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    id: i32,
    name: String,
    email: String,
    password: String
}

pub async fn list(db_pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(db_pool.get_ref())
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => {
            eprintln!("Erro ao buscar usuários: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
