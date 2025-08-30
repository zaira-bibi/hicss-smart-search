use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

fn default_top_k() -> Option<u32> {
    Some(1)
}

fn default_meta_data() -> Option<bool> {
    Some(true)
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct SearchRequest {
    #[schemars(description = "The search query string")]
    pub query: String,
    #[schemars(
        description = "The number of matching results to fetch (ordered descending in terms of relevance.)"
    )]
    #[serde(default = "default_top_k")]
    pub top_k: Option<u32>,
    #[schemars(description = "The flag to get whole metadata of paper")]
    #[serde(default = "default_meta_data")]
    pub full_metadata: Option<bool>,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct AuthorSearchRequest {
    #[schemars(description = "The name of the author whose papers are to be searched.")]
    pub author: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct PapersPayload {
    #[schemars(description = "List of papers to be uploaded.")]
    pub files: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, JsonSchema)]
pub struct SearchDocumentRequest {
    #[schemars(description = "The ID of the document to be searched.")]
    pub doc_id: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct KeywordRequest {
    #[schemars(description = "The keyword query string")]
    pub keyword_query: String,
    #[schemars(
        description = "The number of matching results to fetch (ordered descending in terms of relevance.)"
    )]
    #[serde(default = "default_top_k")]
    pub top_k: Option<u32>,
}
