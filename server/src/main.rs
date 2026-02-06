use axum::{
    Router,
    routing::{post, get},
    response::Json,
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

mod handlers;

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // Build router
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/compress", post(handlers::compress))
        .route("/convert", post(handlers::convert))
        .route("/inspect", post(handlers::inspect))
        .route("/extract", post(handlers::extract))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    // Server address
    let addr = "0.0.0.0:3000";
    log::info!("🚀 Image Preparer Server running on http://{}", addr);
    log::info!("📖 API endpoints:");
    log::info!("   POST /compress - Compress images/videos");
    log::info!("   POST /convert - Convert between formats");
    log::info!("   POST /inspect - View metadata");
    log::info!("   POST /extract - Extract video frames");
    log::info!("   GET  /health - Health check");

    // Start server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Image Preparer Server v0.1.0\n\nAPI Endpoints:\n  POST /compress\n  POST /convert\n  POST /inspect\n  POST /extract\n  GET  /health\n"
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "version": "0.1.0"
    }))
}
