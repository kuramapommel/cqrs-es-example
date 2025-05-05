mod application_services;
mod daos;
mod deserializers;
mod handlers;

use deserializers::deserializer::Deserializer;
use handlers::reservations::create_reservation_handlers;
use handlers::{event_handler::EventHandler, table_managements::create_table_management_handlers};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPoolOptions;
use std::{env, sync::Arc};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
struct AttributeValue {
    s: Option<String>,
    // n: Option<String>,
    b: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DynamoDBStreamRecord {
    new_image: Option<std::collections::HashMap<String, AttributeValue>>,
}

/// This is a made-up example. Requests come into the runtime as unicode
/// strings in json format, which can map to any structure that implements `serde::Deserialize`
/// The runtime pays no attention to the contents of the request payload.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    dynamodb: DynamoDBStreamRecord,
}

/// This is a made-up example of what a response structure may look like.
/// There is no restriction on what it can be. The runtime requires responses
/// to be serialized into json. The runtime pays no attention
/// to the contents of the response payload.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    status_code: i32,
    body: String,
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

    let database_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL env var");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let handlers: Vec<Arc<dyn EventHandler>> = [
        create_reservation_handlers(&pool),
        create_table_management_handlers(&pool),
    ]
    .concat();

    // AWS Lambda のエントリポイント
    let event_handler = Box::new(move |event: LambdaEvent<Request>| {
        let (payload, manifest) =
            Deserializer::deserialize(event.payload).expect("Failed to deserialize payload");

        handlers
            .iter()
            .find_map(|h| h.handle_or_none(&payload, &manifest))
            .expect("No suitable handler found")
    });
    run(service_fn(event_handler)).await
}
