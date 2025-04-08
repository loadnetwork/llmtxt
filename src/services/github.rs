use crate::utils::constants::{GITHUB_API_URL};
use crate::utils::env_var::get_env_var;
use crate::utils::helpers::is_text_file;
use anyhow::{anyhow, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Serialize, Deserialize)]
struct GithubContent {
    name: String,
    path: String,
    #[serde(rename = "type")]
    content_type: String,
    download_url: Option<String>,
    size: Option<u64>,
}

type BoxedFuture<'a> = Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>;

pub async fn download_repo_as_markdown(username: &str, repo: &str) -> Result<String> {
    let client = Client::new();
    let mut markdown = String::new();
    
    markdown.push_str(&format!("# Repository: {}/{}\n\n", username, repo));
    
    // Recursively fetch and process all files
    let contents = fetch_repo_contents(&client, username, repo, "").await?;
    process_contents(&client, username, repo, contents, &mut markdown).await?;
    
    Ok(markdown)
}

pub async fn download_repo_as_text(username: &str, repo: &str) -> Result<String> {
    let client = Client::new();
    let mut text = String::new();
    
    text.push_str(&format!("Repository: {}/{}\n\n", username, repo));
    
    // Recursively fetch and process all files
    let contents = fetch_repo_contents(&client, username, repo, "").await?;
    process_contents_as_text(&client, username, repo, contents, &mut text).await?;
    
    Ok(text)
}

async fn fetch_repo_contents(
    client: &Client,
    username: &str,
    repo: &str,
    path: &str,
) -> Result<Vec<GithubContent>> {
    let github_token = get_env_var("GITHUB_TOKEN")?;
    let url = if path.is_empty() {
        format!("{}/{}/{}/contents", GITHUB_API_URL, username, repo)
    } else {
        format!("{}/{}/{}/contents/{}", GITHUB_API_URL, username, repo, path)
    };

    let mut request = client
        .get(&url)
        .header("User-Agent", "repo-downloader");
    request = request.header("Authorization", format!("token {}", github_token));
    
    
    let response = request.send().await?;

    if !response.status().is_success() {
        return Err(anyhow!(
            "Failed to fetch repository contents: {}",
            response.status()
        ));
    }

    let contents: Vec<GithubContent> = response.json().await?;
    Ok(contents)
}

fn process_contents<'a>(
    client: &'a Client,
    username: &'a str,
    repo: &'a str,
    contents: Vec<GithubContent>,
    markdown: &'a mut String,
) -> BoxedFuture<'a> {
    Box::pin(async move {
        let github_token = get_env_var("GITHUB_TOKEN")?;
        for content in contents {
            if content.content_type == "dir" {
                markdown.push_str(&format!("## Directory: {}\n\n", content.path));
                
                let dir_contents = fetch_repo_contents(client, username, repo, &content.path).await?;
                process_contents(client, username, repo, dir_contents, markdown).await?;
            } else if content.content_type == "file" {
                let file_size_info = content.size.map_or(String::new(), |size| {
                    format!(" ({})", crate::utils::helpers::format_file_size(size))
                });
                
                markdown.push_str(&format!("### File: {}{}\n\n", content.path, file_size_info));
                
                if let Some(download_url) = &content.download_url {
                    if is_text_file(&content.path) {
                        let mut file_request = client
                            .get(download_url)
                            .header("User-Agent", "repo-downloader");
                        
                            file_request = file_request.header("Authorization", format!("token {}", github_token));

                        
                        let file_response = file_request.send().await?;
                        
                        if file_response.status().is_success() {
                            let file_content = file_response.text().await?;
                            
                            let extension = content.path.split('.').last().unwrap_or("");
                            let language_hint = if !extension.is_empty() { extension } else { "" };
                            
                            markdown.push_str(&format!("```{}\n", language_hint));
                            markdown.push_str(&file_content);
                            if !file_content.ends_with('\n') {
                                markdown.push('\n');
                            }
                            markdown.push_str("```\n\n");
                        } else {
                            markdown.push_str(&format!("Failed to download file content: {}\n\n", download_url));
                        }
                    } else {
                        markdown.push_str(&format!("Binary file, content not included. [Download link]({})\n\n", download_url));
                    }
                } else {
                    markdown.push_str("No download URL available for this file.\n\n");
                }
            }
        }
        
        Ok(())
    })
}


fn process_contents_as_text<'a>(
    client: &'a Client,
    username: &'a str,
    repo: &'a str,
    contents: Vec<GithubContent>,
    text: &'a mut String,
) -> BoxedFuture<'a> {
    Box::pin(async move {
        let github_token = get_env_var("GITHUB_TOKEN")?;
        for content in contents {
            if content.content_type == "dir" {
                text.push_str(&format!("Directory: {}\n\n", content.path));

                let dir_contents = fetch_repo_contents(client, username, repo, &content.path).await?;
                process_contents_as_text(client, username, repo, dir_contents, text).await?;
            } else if content.content_type == "file" {

                let file_size_info = content.size.map_or(String::new(), |size| {
                    format!(" ({})", crate::utils::helpers::format_file_size(size))
                });
                
                text.push_str(&format!("File: {}{}\n", content.path, file_size_info));
                text.push_str(&format!("{}\n", "-".repeat(content.path.len() + 6)));
                
                if let Some(download_url) = &content.download_url {
                    if is_text_file(&content.path) {
                        let mut file_request = client
                            .get(download_url)
                            .header("User-Agent", "repo-downloader");
                        
                            file_request = file_request.header("Authorization", format!("token {}", github_token));

                        
                        let file_response = file_request.send().await?;
                        
                        if file_response.status().is_success() {
                            let file_content = file_response.text().await?;
                            
                            text.push_str(&file_content);
                            if !file_content.ends_with('\n') {
                                text.push('\n');
                            }
                            text.push_str("\n\n");
                        } else {
                            text.push_str(&format!("Failed to download file content: {}\n\n", download_url));
                        }
                    } else {
                        text.push_str(&format!("Binary file, content not included. Download URL: {}\n\n", download_url));
                    }
                } else {
                    text.push_str("No download URL available for this file.\n\n");
                }
            }
        }
        
        Ok(())
    })
}
