use crate::utils::constants::LOAD0_ENDPOINT_URL;
use anyhow::{Error, anyhow};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Load0UploadResponse {
    pub optimistic_hash: String,
    pub success: bool,
}

pub async fn upload_to_load0(data: Vec<u8>, content_type: &str) -> Result<String, Error> {
    let client = Client::new();

    let upload_url = format!("{}/upload", LOAD0_ENDPOINT_URL);

    let response = client
        .post(&upload_url)
        .header("Content-Type", content_type)
        .body(data)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Failed to upload to Load0: Status {}, Body: {}",
            response.status(),
            response.text().await?
        ));
    }

    let upload_response = response.json::<Load0UploadResponse>().await?;

    if (upload_response.success) {
        return Ok(upload_response.optimistic_hash);
    }

    Ok(String::from(
        "0x0000000000000000000000000000000000000000000000000000000000000000",
    ))
}
