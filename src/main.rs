use actix_cors::Cors;
use actix_web::{get, http, App, HttpResponse, HttpServer, Responder};
use connection::establish_connection::establish_connection;

pub mod connection;
pub mod models;
pub mod roberta_model;
pub mod schema;
pub mod users;

#[get("sign_up")]
pub async fn sign_up() -> impl Responder {
    HttpResponse::Ok().body("sign up successfully")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    establish_connection();
    HttpServer::new(|| {
        App::new().wrap(
            Cors::default()
                .allowed_origin("http://localhost:4200")
                .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
                .allowed_headers(vec![
                    http::header::AUTHORIZATION,
                    http::header::CONTENT_TYPE,
                ])
                .max_age(3600),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
