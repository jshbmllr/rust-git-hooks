extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate tokio;

mod api;
mod logger;

use hyper::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use hyper::{Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_json::json;
use api::ApiResponse;

#[tokio::main]
async fn request() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let endpoint = "https://api.openai.com/v1/chat/completions";
    let api_key = dotenv::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    // Create an HTTPS connector and set up the client
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Define the payload for the API
    let payload = serde_json::to_string(&json!({
      "model": "gpt-3.5-turbo",
      "messages": [{"role": "user", "content": "Say this is a test!"}],
      "temperature": 0.7
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
    let body_bytes = hyper::body::to_bytes(res).await?;
    let response: ApiResponse = serde_json::from_slice(&body_bytes)?;
    let content = &response.choices[0].message.content.clone();

    // Print the result
    println!("{:?}", content);

    Ok(())
}
