pub mod handler;
pub mod middleware;

use axum::{Router, Server};
use log::info;
use std::net::SocketAddr;

pub const DEFAULT_PORT: u16 = 8000;

pub async fn run(port: Option<u16>) {
    let app = Router::new()
        .merge(handler::frontend_router());

    info!("starting webserver on port {}", port.unwrap_or(DEFAULT_PORT));
    Server::bind(&SocketAddr::from((
        [0, 0, 0, 0],
        port.unwrap_or(DEFAULT_PORT),
    )))
    .serve(app.into_make_service_with_connect_info::<SocketAddr>())
    .await
    .unwrap();
}
