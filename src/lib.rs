pub mod models;

use async_once_cell::OnceCell;
use aws_sdk_cloudwatch::Client;
use dotenv::dotenv;
use models::Ec2MetricStatistics;
use once_cell::sync::Lazy;
use reqwest::Method;
use serde::Deserialize;

pub async fn get_legacy_stack_api() -> Result<f64, String> {
    let endpoint = "https://legacy-stack.yukinissie.com/metrics/";
    let client = reqwest::Client::new();
    let Ok(response) = client.get(endpoint).send().await else {
        return Err("Failed to http request".to_string());
    };

    let Ok(json) = response.json::<Ec2MetricStatistics>().await else {
        return Err("Failed to json parse".to_string());
    };

    Ok(json.cpuutilization)
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub region: String,
    pub instance_id: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    if cfg!(debug_assertions) {
        if let Err(_) = dotenv() {};
    }
    envy::from_env().unwrap()
});

pub static CLIENT: OnceCell<Client> = async_once_cell::OnceCell::new();
