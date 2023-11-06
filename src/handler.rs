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

pub async fn read_quotes(
    extract::State(pool): extract::State<PgPool>,
) -> Result<axum::Json<Vec<Quote>>, http::StatusCode> {
    let res = sqlx::query_as::<_, Quote>(
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
) -> Result<http::StatusCode, http::StatusCode> {
    println!("{:?}", payload);

    let now = Utc::now();

    let res = sqlx::query(
        r#"
        UPDATE quotes
        SET author = $1, quote = $2, updated_at = $3
        WHERE id = $4
        "#,
    )
    .bind(&payload.author)
    .bind(&payload.quote)
    .bind(&now)  // Change here: bind `now` as a reference
    .bind(&id)   // Change here: bind `id` as a reference
    .execute(&pool)
    .await
    .map(|res| {
        if res.rows_affected() == 0 {
            http::StatusCode::NOT_FOUND
        } else {
            http::StatusCode::OK
        }
    });

    match res {
        Ok(status) => Ok(status),
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

