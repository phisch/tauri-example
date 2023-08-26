use axum::{
    body::Body,
    http::{header, Response, StatusCode, Uri},
    routing::get,
    Extension, Router,
};
use client::{AssetKey, Assets, EmbeddedAssets};
use std::sync::Arc;

pub fn get_router() -> Router {
    Router::new()
        .route("/", get(index))
        .fallback_service(get(frontend_assets))
        .layer(Extension(client::get_context().assets()))
}

async fn index(Extension(assets): Extension<Arc<EmbeddedAssets>>) -> Response<Body> {
    match assets.get(&AssetKey::from("/index.html")) {
        Some(asset) => Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "text/html")
            .body(Body::from(asset.to_vec()))
            .unwrap(),
        None => Response::new(Body::from("404")),
    }
}

async fn frontend_assets(
    Extension(assets): Extension<Arc<EmbeddedAssets>>,
    uri: Uri,
) -> Response<Body> {
    match assets.get(&AssetKey::from(uri.path())) {
        Some(asset) => {
            let mime = mime_guess::from_path(uri.path());
            let mime_type = mime.first().unwrap().to_string();
            let body = Body::from(asset.to_vec());
            Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, mime_type)
                .body(body)
                .unwrap()
        }
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .unwrap()
    }
}
