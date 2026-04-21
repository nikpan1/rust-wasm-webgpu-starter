use axum::{
    body::Body,
    http::Request,
    middleware::{self, Next},
    response::Response,
    Router,
};
use tower_http::services::ServeDir;

/// Injects the COOP/COEP headers required for SharedArrayBuffer and
/// recommended for WebGPU workloads.
async fn add_security_headers(request: Request<Body>, next: Next) -> Response {
    let mut response = next.run(request).await;
    let h = response.headers_mut();
    h.insert(
        "cross-origin-opener-policy",
        "same-origin".parse().unwrap(),
    );
    h.insert(
        "cross-origin-embedder-policy",
        "require-corp".parse().unwrap(),
    );
    response
}

#[tokio::main]
async fn main() {
    // ── Resolve client/dist relative to this crate (works from any CWD) ──
    let dist = concat!(env!("CARGO_MANIFEST_DIR"), "/../client/dist");

    // ── Static file service ───────────────────────────────────────────────
    // ServeDir handles:
    // - Correct MIME types (incl. application/wasm for .wasm files)
    // - Serving index.html for directory requests
    let serve_dir = ServeDir::new(dist).append_index_html_on_directories(true);

    let app = Router::new()
        .nest_service("/", serve_dir)
        .layer(middleware::from_fn(add_security_headers));

    // ── Start listener ─────────────────────────────────────────────────────
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!();
    println!("  ┌─────────────────────────────────────────┐");
    println!("  │  🦀 Rust × WebGPU Showcase              │");
    println!("  │                                         │");
    println!("  │     http://localhost:3000               │");
    println!("  │                                         │");
    println!("  │  Press Ctrl+C to stop                   │");
    println!("  └─────────────────────────────────────────┘");
    println!();

    axum::serve(listener, app).await.unwrap();
}
