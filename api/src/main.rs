use axum::{
    response::IntoResponse,
    routing::{get, MethodRouter},
    Json, Router,
};
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "api=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with a single route
    let app = Router::new()
        .merge(get_root())
        // logging so we can see whats going on
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(false)),
        );

    // run it with hyper on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn get_root() -> Router {
    async fn get_root() -> impl IntoResponse {
        let id = Uuid::new_v4();
        let hello = "hello from axum!".to_string();
        let resp = HelloResponse { hello, id };
        Json(resp)
    }

    route("/", get(get_root))
}

#[derive(Debug, Serialize, Clone)]
struct HelloResponse {
    hello: String,
    id: Uuid,
}

fn route(path: &str, method_router: MethodRouter) -> Router {
    Router::new().route(path, method_router)
}
