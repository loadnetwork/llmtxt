pub mod services;
pub mod utils;

use shuttle_axum::axum::{routing::get, Router};
use crate::services::handlers::{llm_txt_handler, status_handler};


#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    unsafe {
        secrets.into_iter().for_each(|(key, val)| {
            println!("{:?} {:?}", key, val);
            std::env::set_var(key, val);
        });
    }

    let router = Router::new()
        .route("/", get(status_handler))
        .route("/port/{user}/{repo_name}", get(llm_txt_handler));
    Ok(router.into())
}
