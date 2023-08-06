use lambda_flows::{request_received, send_response};
use flowsnet_platform_sdk::logger;
use std::collections::HashMap;
use serde_json::Value;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    request_received(|headers, qry, body| {
        handler(headers, qry, body)
    }).await;
    Ok(())
}

async fn handler(headers: Vec<(String, String)>, qry: HashMap<String, Value>, _body: Vec<u8>) {
    logger::init();
    log::info!("Headers -- {:?}", headers);

    // Extract username and password from the query parameters
    let username = qry.get("username").and_then(Value::as_str).unwrap_or("");
    let password = qry.get("password").and_then(Value::as_str).unwrap_or("");

    // Check if the provided username and password are valid 
    let login_status = if username == "Zekrom7780" && password == "Random123" {
        "success"
    } else {
        "failure"
    };

    // Create a JSON response
    let resp_json = serde_json::json!({
        "status": login_status,
        "message": format!("Login {}.", login_status),
    });

    // Set the response content type to application/json
    send_response(
        200,
        vec![(String::from("content-type"), String::from("application/json"))],
        resp_json.to_string().as_bytes().to_vec(),
    );
}
