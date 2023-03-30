mod config;
mod database;
mod http;

use actix_cors::Cors;
use actix_web::http::header;
use actix_web::web::Data;
use actix_web::{guard, web, App, HttpServer};
use database::core::connect;
use http::handler::{graphql_handler, playground_handler};
use http::schema::build_schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = build_schema();
    let pool = connect().await;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin_fn(|_origin, _req_head| true)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(pool.clone()))
            .service(web::resource("/").guard(guard::Post()).to(graphql_handler))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .to(playground_handler),
            )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
