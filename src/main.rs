mod conn;

use actix_web::{web, App, HttpServer, Responder};
use dotenv::dotenv;
use conn::connect;

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let pool = connect().await?;

    HttpServer::new(|| {
        App::new().service(
            web::scope("/app")
                .route("/index.html", web::get().to(index)),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}