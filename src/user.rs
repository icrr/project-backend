use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use serde::Serialize;

#[derive(sqlx::FromRow, Serialize)]
pub struct Users {
    id: i32,
    name: String,
    email: String,
    password: String
}

pub async fn list(db_pool: web::Data<PgPool>) -> HttpResponse {
    match sqlx::query_as::<_, Users>("SELECT * FROM users")
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

#[derive(sqlx::FromRow, Serialize)]
pub struct CreateUser {
    name: String,
    email: String,
    password: String
}

pub async fn create(db_pool: web::Data<PgPool>, web_form: web::Form<CreateUser>,) -> HttpResponse {
    let new_user = web_form.into_inner();
    
    let query = sqlx::query(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id, name, email",
    )
    .bind(&new_user.name)
    .bind(&new_user.email)
    .bind(&new_user.password);
}

#[derive(sqlx::FromRow, Serialize)]
pub struct LoginUser {
    email: String,
    password: String
}