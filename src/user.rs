use actix_web::{web, HttpResponse};
use sqlx::{
    PgPool,
    Row,
};
use serde::{
    Serialize,
    Deserialize,
};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Users {
    id: i32,
    name: String,
    email: String,
    password: String,
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

#[derive(Serialize)]
pub struct CreatedUser {
    id: i32,
    name: String,
    email: String,
}

#[derive(Deserialize)]
pub struct CreateUser {
    name: String,
    email: String,
    password: String,
}

pub async fn create(db_pool: web::Data<PgPool>, web_form: web::Form<CreateUser>) -> HttpResponse {
    let new_user = web_form.into_inner();

    let query = sqlx::query(
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING id, name, email",
    )
    .bind(&new_user.name)
    .bind(&new_user.email)
    .bind(&new_user.password);

    match query.fetch_one(db_pool.get_ref()).await {
        Ok(user) => {
            let created_user = CreatedUser {
                id: user.get("id"),
                name: user.get("name"),
                email: user.get("email"),
            };
            HttpResponse::Created().json(created_user)
        }
        Err(e) => {
            eprintln!("Erro ao criar usuário: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn delete(db_pool: web::Data<PgPool>, path: web::Path<i32>) -> HttpResponse {
    let userd_id = path.into_inner();

    let query = sqlx::query(
        "DELETE FROM users WHERE id = $1 RETURNING id, name, email",
    )
    .bind(userd_id);

    match query.fetch_one(db_pool.get_ref()).await {
        Ok(user) => {
            let deleted_user = Users {
                id: user.get("id"),
                name: user.get("name"),
                email: user.get("email"),
                password: user.get("password"),
            };
            HttpResponse::Ok().json(deleted_user)
        }
        Err(e) => {
            eprintln!("Erro ao excluir usuário: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}