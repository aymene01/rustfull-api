mod handlers;
use axum::routing::{get, Router, post, put, delete};
use std::env;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port: String = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr: String = format!("0.0.0.0:{}", port);

    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let app: Router = Router::new()
        .route("/", get(handlers::health))
        .route("/quotes", post(handlers::create_quote))
        .route("/quotes", get(handlers::read_quotes))
        .route("/quotes/:id", put(handlers::update_quote))
        .route("/quotes/:id", delete(handlers::delete_quote))
        .with_state(pool);

    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

