mod backend;
mod frontend;

use axum::{response::Html, routing::get, Router};
use backend::{
    db::Database,
    handlers::{create_router, AppState},
    service::EvaluationService,
};
use std::sync::Arc;

const DATABASE_URL: &str = "sqlite://daily_checksheet.db?mode=rwc";
const BIND_ADDR: &str = "127.0.0.1:5113";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db = Database::new(DATABASE_URL)
        .await
        .expect("failed to initialize database");
    let service = Arc::new(EvaluationService::new(db));
    
    // Preload tests
    service.preload_tests().await;

    let state = Arc::new(AppState { service });

    let app = Router::new()
        .route("/", get(|| async { Html(frontend::get_index_html()) }))
        .merge(create_router(state));

    let listener = tokio::net::TcpListener::bind(BIND_ADDR)
        .await
        .expect("failed to bind address");

    println!("✅ daily_checksheet is running");
    println!("🌐 Access at: http://{BIND_ADDR}");

    axum::serve(listener, app).await.expect("server error");
}
