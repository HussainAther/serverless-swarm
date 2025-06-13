use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};
use aws_sdk_dynamodb::{Client, types::AttributeValue};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Incoming {
    user_id: String,
}

#[derive(Serialize)]
struct Outgoing {
    message: String,
    user_id: String,
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    // Extract query param (e.g., /users?user_id=123)
    let user_id = event
        .query_string_parameters()
        .get("user_id")
        .unwrap_or("unknown")
        .to_string();

    // OR extract JSON body
    // let incoming: Incoming = match event.payload::<Incoming>() {
    //     Ok(Some(data)) => data,
    //     _ => return Ok(Response::builder().status(400).body("Invalid JSON".into())?),
    // };

    // DynamoDB client (uses environment config)
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    // Put item into table
    let table_name = std::env::var("USERS_TABLE").unwrap_or("usersTable".to_string());

    match client
        .put_item()
        .table_name(&table_name)
        .item("user_id", AttributeValue::S(user_id.clone()))
        .item("created_at", AttributeValue::S(chrono::Utc::now().to_rfc3339()))
        .send()
        .await
    {
        Ok(_) => {
            let body = serde_json::to_string(&Outgoing {
                message: "User added successfully".to_string(),
                user_id,
            })?;
            Ok(Response::builder().status(200).body(body.into())?)
        }
        Err(e) => {
            eprintln!("DynamoDB error: {:?}", e);
            Ok(Response::builder().status(500).body("Internal Server Error".into())?)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}

