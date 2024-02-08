use actix_web::{web, App, HttpServer};
use actix_web::web::Data;
mod user;
use user::{list, put_name, delete, create};
mod conn;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = conn::_connect().await.expect("Failed to connect to the database.");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_pool.clone()))
            .service(web::scope("/app")
                .route("/users", web::get().to(list))
                .route("/users", web::post().to(create))
                .route("/users/{id}", web::delete().to(delete))
                .route("/users/{id}", web::put().to(put_name))
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

