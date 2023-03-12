use async_once_cell::OnceCell;
use aws_sdk_dynamodb::Client;
use dotenv::dotenv;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub region: String,
    pub table_name: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    if cfg!(debug_assertions) {
        if let Err(_) = dotenv() {};
    }
    envy::from_env().unwrap()
});

pub static CLIENT: OnceCell<Client> = async_once_cell::OnceCell::new();
