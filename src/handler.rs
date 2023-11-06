use axum::{http, extract};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{pool, PgPool};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Quote {
    id: uuid::Uuid,
    author: String,
    quote: String,
    inserted_at: chrono::DateTime<Utc>,
    updated_at: chrono::DateTime<Utc>,
}

impl Quote {
    pub fn new(author: String, quote: String) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            author,
            quote,
            inserted_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateQuote {
    pub author: String,
    pub quote: String,
}

pub async fn health() -> http::StatusCode {
    http::StatusCode::OK
}

pub async fn create_quote(extract::State(pool): extract::State<PgPool>, axum::Json(payload): axum::Json<CreateQuote>) -> Result<(http::StatusCode, axum::Json<Quote>), http::StatusCode> {
    println!("{:?}", payload);
    let quote = Quote::new(payload.author.clone(), payload.quote.clone());

    let res = sqlx::query(
        r#"
        INSERT INTO quotes (id, author, quote, inserted_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(&quote.id)
    .bind(&quote.author)
    .bind(&quote.quote)
    .bind(&quote.inserted_at)
    .bind(&quote.updated_at)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok((http::StatusCode::CREATED, axum::Json(quote))),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
