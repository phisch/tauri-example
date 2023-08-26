use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{
    extract::ConnectInfo,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn drop_external<B: Send>(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let is_localhost = addr.ip() == IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    if headers.get(axum::http::header::FORWARDED).is_none() && is_localhost {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
