use std::env;
mod post;
mod user;

use actix_cors::Cors;
use actix_web::{http::header, middleware::Logger, web::Data, App, HttpServer};
use log::{error, info};
use post::service::post_service;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use user::service::user_service;

#[derive(Clone)]
pub struct SharedState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    info!("Backend started");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match PgPoolOptions::new()
        .max_connections(3)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            info!("Connection to db succeded!");
            pool
        }
        Err(err) => {
            error!("Connection to DB failed: {}", err);
            std::process::exit(1)
        }
    };

    sqlx::migrate!();
    HttpServer::new(move || {
        let cors = Cors::default()
            // .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(Data::new(SharedState { db: pool.clone() }))
            .configure(user_service)
            .configure(post_service)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;
    Ok(())
}
