extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate tokio;

mod api;

use hyper::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_json::json;
use api::ChatResponse;

#[tokio::main]
pub async fn request(request_content: String) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let endpoint = "https://api.cohere.ai/v1/chat";
    let api_key = dotenv::var("CO_API_KEY").expect("CO_API_KEY must be set");

    // Create an HTTPS connector and set up the client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Define the payload for the API
    let payload = serde_json::to_string(&json!({
        "chat_history": [{"role":"system","message": "You are a service that accepts git diffs and returns a concise commit message based on them"},
                         {"role": "user", "message": request_content}
                        ],
        "message": request_content,
    }))?;

    // Construct the request
    let req = Request::builder()
        .method("POST")
        .uri(endpoint)
        .header(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", api_key))?,
        )
        .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
        .body(Body::from(payload))?;
    
    // Send the request and get the response
    let res = client.request(req).await?;
    println!("{:?}", res);
    let body_bytes = hyper::body::to_bytes(res).await?;
    let response: ChatResponse = serde_json::from_slice(&body_bytes)?;
    let content = &response.text.clone();

    // Print the result
    println!("{:?}", content);

    Ok(content).cloned()
}
