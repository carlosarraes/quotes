use crate::controllers::quotes;
use crate::controllers::handlers;
use axum::{
    routing::{ get, post, put, delete }, 
    Router
};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{port}");

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    println!("Connecting to PostgreSQL");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    let app = Router::new()
        .route("/", get(handlers::health))
        .route("/quotes", get(quotes::get_all))
        .route("/quotes/:id", get(quotes::get))
        .route("/quotes", post(quotes::create))
        .route("/quotes/:id", put(quotes::update))
        .route("/quotes/:id", delete(quotes::delete))
        .with_state(pool);

    println!("Server running on port {port}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

mod controllers {
    pub mod handlers;
    pub mod quotes;
}
