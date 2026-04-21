use axum::{Router, response::Html};
use tower_http::services::ServeDir;

/// Placeholder server — will serve client/dist in Phase 5.
#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/", ServeDir::new("client/dist"))
        .fallback(|| async { Html("<h1>Building…</h1>") });

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Serving on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
