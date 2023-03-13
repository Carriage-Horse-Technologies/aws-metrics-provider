use aws_config::meta::region::RegionProviderChain;
use aws_metrics_provider::{models::Ec2MetricStatistics, CLIENT, CONFIG};
use aws_sdk_cloudwatch::{
    model::{Datapoint, Dimension, Statistic},
    Client, Region,
};
use chrono::{DateTime, Duration, Utc};
use fancy_regex::Regex;
use lambda_http::{
    http::Method, request::RequestContext, run, service_fn, Body, Error, Request, RequestExt,
    Response,
};
use once_cell::sync::Lazy;
use serde_json::json;

async fn get_metric_statistics(
    metric_name: &str,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
) -> Datapoint {
    let client = CLIENT.get().unwrap();

    let dimension = Dimension::builder()
        .set_name(Some("InstanceId".to_string()))
        .set_value(Some(CONFIG.instance_id.to_string()))
        .build();

    let metric_statistics = client
        .get_metric_statistics()
        .set_start_time(Some(aws_smithy_types::DateTime::from_millis(
            start.timestamp_millis(),
        )))
        .set_end_time(Some(aws_smithy_types::DateTime::from_millis(
            end.timestamp_millis(),
        )))
        .set_statistics(Some(vec![Statistic::Average]))
        .set_namespace(Some("AWS/EC2".to_string()))
        .set_metric_name(Some(metric_name.to_string()))
        .set_period(Some(3600))
        .set_dimensions(Some(vec![dimension]))
        .send()
        .await
        .expect("Failed to get_metric_statistics");

    metric_statistics.datapoints().unwrap()[0].clone()
}

async fn get_cpuutilization(start: DateTime<Utc>, end: DateTime<Utc>) -> Datapoint {
    get_metric_statistics("CPUUtilization", start, end).await
}

async fn get_disk_read_bytes(start: DateTime<Utc>, end: DateTime<Utc>) -> Datapoint {
    get_metric_statistics("DiskReadBytes", start, end).await
}

async fn get_disk_write_bytes(start: DateTime<Utc>, end: DateTime<Utc>) -> Datapoint {
    get_metric_statistics("DiskWriteBytes", start, end).await
}

async fn get_network_in(start: DateTime<Utc>, end: DateTime<Utc>) -> Datapoint {
    get_metric_statistics("NetworkIn", start, end).await
}

async fn get_network_out(start: DateTime<Utc>, end: DateTime<Utc>) -> Datapoint {
    get_metric_statistics("NetworkOut", start, end).await
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request

    let end = chrono::Utc::now();
    let start = end - Duration::hours(1);

    let ec2_metric_statistics = Ec2MetricStatistics {
        cpuutilization: get_cpuutilization(start, end)
            .await
            .average()
            .unwrap_or_default(),
        disk_read_bytes: get_disk_read_bytes(start, end)
            .await
            .average()
            .unwrap_or_default(),
        disk_write_bytes: get_disk_write_bytes(start, end)
            .await
            .average()
            .unwrap_or_default(),
        network_in: get_network_in(start, end)
            .await
            .average()
            .unwrap_or_default(),
        network_out: get_network_out(start, end)
            .await
            .average()
            .unwrap_or_default(),
    };

    let response = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(
            serde_json::to_string(&ec2_metric_statistics)
                .unwrap_or_default()
                .into(),
        )
        .map_err(Box::new)?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let region_provider = RegionProviderChain::first_try(Region::new(&CONFIG.region))
        .or_default_provider()
        .or_else(Region::new(&CONFIG.region));

    CLIENT
        .get_or_init(async {
            let shared_config = aws_config::from_env().region(region_provider).load().await;
            Client::new(&shared_config)
        })
        .await;

    run(service_fn(function_handler)).await
}
