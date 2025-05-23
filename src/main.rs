pub mod services;
pub mod utils;

use crate::services::handlers::{llm_txt_handler, status_handler};
use shuttle_axum::axum::{
    Router,
    http::{Method, header},
    routing::get,
};
use std::time::Duration;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    unsafe {
        secrets.into_iter().for_each(|(key, val)| {
            std::env::set_var(key, val);
        });
    }

    let timeout_layer = TimeoutLayer::new(Duration::from_secs(3600));
    let cors_layer = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION, header::ACCEPT]);

    let router = Router::new()
        .route("/", get(status_handler))
        .route("/port/{user}/{repo_name}", get(llm_txt_handler))
        .layer(timeout_layer)
        .layer(cors_layer);
    Ok(router.into())
}
