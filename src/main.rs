use aws_sdk_dynamodb::Client;
use fancy_regex::Regex;
use lambda_http::{
    http::Method, request::RequestContext, run, service_fn, Body, Error, Request, RequestExt,
    Response,
};
use once_cell::sync::Lazy;
use rust_lambda::CLIENT;
use serde_json::json;

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract some useful information from the request

    let RequestContext::ApiGatewayV2(http_context) = event.request_context();

    let resource_path = http_context.http.path.unwrap();
    tracing::info!("{}", resource_path);

    // static Regex: Lazy<Regex> = Lazy::new(|| Regex::new("^/\\w+(?=/)").unwrap());
    // // stage名を削除
    // let resource_path = RATE.replace(&resource_path, "");

    let resp = match (event.method(), resource_path.as_ref()) {
        (&Method::GET, path) => Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(
                json!({ "message": format!("GET {}", path) })
                    .to_string()
                    .into(),
            )
            .map_err(Box::new)?,
        (&Method::POST, "/users") => Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(
                json!({
                    "message": "POST /users"
                })
                .to_string()
                .into(),
            )
            .map_err(Box::new)?,
        _ => Response::builder()
            .status(200)
            .header("content-type", "application/json")
            .body(
                json!({
                    "message": "This method or path is not support."
                })
                .to_string()
                .into(),
            )
            .map_err(Box::new)?,
    };

    Ok(resp)
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

    CLIENT
        .get_or_init(async {
            let shared_config = aws_config::load_from_env().await;
            Client::new(&shared_config)
        })
        .await;

    run(service_fn(function_handler)).await
}
