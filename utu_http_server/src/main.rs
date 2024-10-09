mod endpoints;
mod errors;
mod logger;
mod state;
mod utils;
use axum::{Extension, Router};
use axum_auto_routes::route;
use reqwest::StatusCode;
use state::AppState;
use std::net::{IpAddr, Ipv4Addr};
use std::{
    env,
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tokio::net::TcpListener;
use tower_http::cors::{self, CorsLayer};
use utils::routing::WithState;
lazy_static::lazy_static! {
    pub static ref ROUTE_REGISTRY: Mutex<Vec<Box<dyn WithState>>> = Mutex::new(Vec::new());
    pub static ref SERVER_PORT: u16 = env::var("SERVER_PORT")
        .expect("SERVER_PORT must be set")
        .parse()
        .expect("SERVER_PORT must be a valid u16");
}

#[tokio::main]
async fn main() {
    let shared_state: Arc<AppState> = AppState::load().await;

    let cors = CorsLayer::new()
        .allow_headers(cors::Any)
        .allow_origin(cors::Any);
    let app = ROUTE_REGISTRY
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .fold(Router::new(), |acc, r| {
            acc.merge(r.to_router(shared_state.clone()))
        })
        .layer(cors)
        .layer(Extension(shared_state.clone()));

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), *SERVER_PORT);
    let listener = TcpListener::bind(addr).await.unwrap();
    shared_state
        .logger
        .async_info(format!(
            "server: listening on http://0.0.0.0:{}",
            *SERVER_PORT
        ))
        .await;
    axum::serve(listener, app.with_state(shared_state))
        .await
        .unwrap();
}

#[route(get, "/")]
async fn root() -> (StatusCode, String) {
    (
        StatusCode::ACCEPTED,
        format!("server v{}", env!("CARGO_PKG_VERSION")),
    )
}
