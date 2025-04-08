use crate::services::github::download_repo_as_text;
use crate::services::load0::upload_to_load0;
use axum::{extract::Path, response::Json};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Debug, Serialize, Deserialize)]
pub struct LlmTxtHandler {
    pub user: String,
    pub repo: String,
    pub load0_hash: String,
    pub size: u32,
}

impl LlmTxtHandler {
    pub fn from(user: String, repo: String, load0_hash: String, size: u32) -> Self {
        Self {
            user,
            repo,
            load0_hash,
            size,
        }
    }
}

pub async fn status_handler() -> Json<Value> {
    Json(json!({"status": "running"}))
}

pub async fn llm_txt_handler(Path((username, repo_name)): Path<(String, String)>) -> Json<Value> {
    let llm_content = download_repo_as_text(&username, &repo_name).await.unwrap();
    let load0_hash = upload_to_load0(llm_content.as_bytes().to_vec(), "text/plain")
        .await
        .unwrap();
    let res = LlmTxtHandler::from(username, repo_name, load0_hash, llm_content.len() as u32);
    Json(serde_json::to_value(res).unwrap())
}
