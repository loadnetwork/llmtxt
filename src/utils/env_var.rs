use std::env;
use dotenv::dotenv;
use anyhow::Error;

pub fn get_env_var(key: &str) -> Result<String, Error> {
    dotenv().ok();
    Ok(env::var(key)?)
}