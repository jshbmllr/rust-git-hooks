use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatResponse {
    pub text: String,
    pub generation_id: Option<String>,
    pub citations: Option<Vec<Citation>>,
    pub documents: Option<Vec<Document>>,
    pub is_search_required: Option<bool>,
    pub search_queries: Option<Vec<SearchQuery>>,
    pub search_results: Option<Vec<SearchResult>>,
    pub finish_reason: Option<String>,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub chat_history: Option<Vec<ChatMessage>>,
    pub meta: Option<Meta>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Citation {
    pub start: Option<usize>,
    pub end: Option<usize>,
    pub text: Option<String>,
    pub document_ids: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Document {
    pub id: Option<String>,
    pub additional_prop: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchQuery {
    pub text: Option<String>,
    pub generation_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResult {
    pub search_query: Option<SearchQuery>,
    pub connector: Option<Connector>,
    pub document_ids: Option<Vec<String>>,
    pub error_message: Option<String>,
    pub continue_on_failure: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Connector {
    pub id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ToolCall {
    pub name: Option<String>,
    pub parameters: Option<HashMap<String, HashMap<String, String>>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChatMessage {
    pub role: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Meta {
    pub api_version: Option<APIVersion>,
    pub billed_units: Option<BilledUnits>,
    pub tokens: Option<Tokens>,
    pub warnings: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct APIVersion {
    pub version: Option<String>,
    pub is_deprecated: Option<bool>,
    pub is_experimental: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BilledUnits {
    pub input_tokens: Option<usize>,
    pub output_tokens: Option<usize>,
    pub search_units: Option<usize>,
    pub classifications: Option<usize>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Tokens {
    pub input_tokens: Option<usize>,
    pub output_tokens: Option<usize>,
}