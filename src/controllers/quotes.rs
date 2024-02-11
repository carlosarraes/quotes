use axum::{http::StatusCode, Json};
use serde::{ Deserialize, Serialize };
use sqlx::PgPool;
use axum::extract;

#[derive(Serialize, Debug)]
pub struct Quote {
    id: uuid::Uuid,
    book: String,
    quote: String,
    inserted_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl Quote {
    fn new(book: String, quote: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            book,
            quote,
            inserted_at: now,
            updated_at: now,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CreateQuote {
    book: String,
    quote: String,
}

pub async fn create(
    extract::State(pool): extract::State<PgPool>,
    Json(payload): Json<CreateQuote>,
) -> Result<(StatusCode, Json<Quote>), StatusCode> {
    let quote = Quote::new(payload.book, payload.quote);

    let res = sqlx::query(
        r#"
        INSERT INTO quotes (id, book, quote, inserted_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    ).bind(&quote.id)
    .bind(&quote.book)
    .bind(&quote.quote)
    .bind(&quote.inserted_at)
    .bind(&quote.updated_at)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok((StatusCode::CREATED, Json(quote))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_all(
    extract::State(pool): extract::State<PgPool>,
) -> Result<Json<Vec<Quote>>, StatusCode> {
    let recs = sqlx::query_as!(
        Quote,
        r#"
        SELECT id, book, quote, inserted_at, updated_at
        FROM quotes
        ORDER BY inserted_at DESC
        "#,
    )
    .fetch_all(&pool)
    .await;

    match recs {
        Ok(recs) => Ok(Json(recs)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get(
    extract::Path(id): extract::Path<uuid::Uuid>,
    extract::State(pool): extract::State<PgPool>,
) -> Result<Json<Quote>, StatusCode> {
    let rec = sqlx::query_as!(
        Quote,
        r#"
        SELECT id, book, quote, inserted_at, updated_at
        FROM quotes
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&pool)
    .await;

    match rec {
        Ok(Some(rec)) => Ok(Json(rec)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update(
    extract::Path(id): extract::Path<uuid::Uuid>,
    extract::State(pool): extract::State<PgPool>,
    Json(payload): Json<CreateQuote>,
) -> Result<Json<Quote>, StatusCode> {
    let quote = Quote::new(payload.book, payload.quote);

    let res = sqlx::query(
        r#"
        UPDATE quotes
        SET book = $1, quote = $2, updated_at = $3
        WHERE id = $4
        "#,
    ).bind(&quote.book)
    .bind(&quote.quote)
    .bind(&quote.updated_at)
    .bind(&id)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok(Json(quote)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete(
    extract::Path(id): extract::Path<uuid::Uuid>,
    extract::State(pool): extract::State<PgPool>,
) -> Result<StatusCode, StatusCode> {
    let res = sqlx::query(
        r#"
        DELETE FROM quotes
        WHERE id = $1
        "#,
    ).bind(&id)
    .execute(&pool)
    .await;

    match res {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
