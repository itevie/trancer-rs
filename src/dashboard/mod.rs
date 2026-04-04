mod auth;

use crate::database::Database;
use axum::{extract::State, routing::get, Router};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

#[derive(Clone)]
struct AppState {
    db: Database,
}

async fn handler(State(_state): State<AppState>) -> String {
    "ok".to_string()
}

pub async fn run_http(db: Database) {
    let app = Router::new()
        .route("/", get(handler))
        .with_state(AppState { db })
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO)),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
