use axum::{http, extract};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};

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
    let quote: Quote = Quote::new(payload.author.clone(), payload.quote.clone());

    let res: Result<sqlx::postgres::PgQueryResult, sqlx::Error> = sqlx::query(
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

pub async fn read_quotes(
    extract::State(pool): extract::State<PgPool>,
) -> Result<axum::Json<Vec<Quote>>, http::StatusCode> {
    let res: Result<Vec<Quote>, sqlx::Error> = sqlx::query_as::<_, Quote>(
        r#"
        SELECT * FROM quotes
        "#,
    )
    .fetch_all(&pool)
    .await;

    match res {
        Ok(quotes) => Ok(axum::Json(quotes)),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_quote(
    extract::Path(id): extract::Path<String>,
    extract::State(pool): extract::State<PgPool>,
    axum::Json(payload): axum::Json<CreateQuote>,
) -> http::StatusCode {
    println!("{:?}", payload);

    let now: chrono::DateTime<Utc> = Utc::now();

    let res: Result<http::StatusCode, sqlx::Error> = sqlx::query(
        r#"
        UPDATE quotes
        SET author = $1, quote = $2, updated_at = $3
        WHERE id = $4
        "#,
    )
    .bind(&payload.author)
    .bind(&payload.quote)
    .bind(now)  
    .bind(id)  
    .execute(&pool)
    .await
    .map(|res| match res.rows_affected() {
        0 => http::StatusCode::NOT_FOUND,
        _ => http::StatusCode::OK,
    });

    match res {
        Ok(status) => status,
        Err(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn delete_quote(
    extract::Path(id): extract::Path<String>,
    extract::State(pool): extract::State<PgPool>,
) -> http::StatusCode {
    let res: Result<http::StatusCode, sqlx::Error> = sqlx::query(
        r#"
        DELETE FROM quotes
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&pool)
    .await
    .map(|res| match res.rows_affected() {
        0 => http::StatusCode::NOT_FOUND,
        _ => http::StatusCode::OK,
    });

    match res {
        Ok(status) => status,
        Err(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
    }
}
