use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponse {
   pub choices: Vec<Choice>,
   pub created: u64,
   pub id: String,
   pub model: String,
   pub object: String,
   pub usage: Usage,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Choice {
   pub  finish_reason: String,
   pub  index: u32,
   pub  message: Message,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
   pub content: String,
   pub role: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Usage {
   pub completion_tokens: u32,
   pub prompt_tokens: u32,
   pub total_tokens: u32,
}
