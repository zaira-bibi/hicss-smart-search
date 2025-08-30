use reqwest::{
    Method,
    multipart::{Form, Part},
};
use rmcp::{
    ErrorData, ServerHandler,
    handler::server::tool::{Parameters, ToolRouter},
    model::{
        CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
    },
    tool, tool_handler, tool_router,
};

use crate::{
    config::{
        SEARCH_BY_AUTHOR_ENDPOINT, SEARCH_BY_DOC_ID_ENDPOINT, SEARCH_BY_KEYWORD_ENDPOINT,
        SEARCH_ENDPOINT, UPLOAD_PAPERS_ENDPOINT,
    },
    search::{
        SearchClient,
        types::{
            AuthorSearchRequest, KeywordRequest, PapersPayload, SearchDocumentRequest,
            SearchRequest,
        },
    },
};

#[derive(Clone)]
pub struct SmartSearchMCP {
    pub client: SearchClient,
    pub tool_router: ToolRouter<Self>,
}

#[tool_router]
impl SmartSearchMCP {
    pub fn new() -> Self {
        Self {
            client: SearchClient::new(),
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Search papers based on the query. Never assume missing values.")]
    pub async fn search(
        &self,
        Parameters(search_query): Parameters<SearchRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        match self
            .client
            .request(
                Method::GET,
                SEARCH_ENDPOINT,
                Some(serde_json::to_value(search_query).unwrap()),
                None,
            )
            .await
        {
            Ok(response) => Ok(CallToolResult::success(vec![
                Content::json(response).unwrap(),
            ])),
            Err(err) => Err(ErrorData::internal_error(
                format!("Error while fetching search results: {err}"),
                None,
            )),
        }
    }

    #[tool(description = "Search papers based on the author's name. Never assume missing values.")]
    pub async fn search_by_author(
        &self,
        Parameters(query): Parameters<AuthorSearchRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        match self
            .client
            .request(
                Method::GET,
                SEARCH_BY_AUTHOR_ENDPOINT,
                Some(serde_json::to_value(query).unwrap()),
                None,
            )
            .await
        {
            Ok(response) => Ok(CallToolResult::success(vec![
                Content::json(response).unwrap(),
            ])),
            Err(err) => Err(ErrorData::internal_error(
                format!("Error while fetching search by author results: {err}"),
                None,
            )),
        }
    }

    #[tool(description = "Upload the papers as string array. Never assume missing values.")]
    pub async fn upload_papers(
        &self,
        Parameters(payload): Parameters<PapersPayload>,
    ) -> Result<CallToolResult, ErrorData> {
        let mut form = Form::new();
        for file in payload.files {
            form = form.part("files[]", Part::text(file));
        }

        match self
            .client
            .request(Method::POST, UPLOAD_PAPERS_ENDPOINT, None, Some(form))
            .await
        {
            Ok(response) => Ok(CallToolResult::success(vec![
                Content::json(response).unwrap(),
            ])),
            Err(err) => Err(ErrorData::internal_error(
                format!("Error while uploading papers: {err}"),
                None,
            )),
        }
    }

    #[tool(description = "Search for the document by the id. Never assume missing values.")]
    pub async fn search_by_doc_id(
        &self,
        Parameters(params): Parameters<SearchDocumentRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        match self
            .client
            .request(
                Method::GET,
                SEARCH_BY_DOC_ID_ENDPOINT,
                Some(serde_json::to_value(params.clone()).unwrap()),
                None,
            )
            .await
        {
            Ok(response) => Ok(CallToolResult::success(vec![
                Content::json(response).unwrap(),
            ])),
            Err(err) => Err(ErrorData::internal_error(
                format!("Error searching for document {}: {err}", params.doc_id),
                None,
            )),
        }
    }

    #[tool(description = "Search for sections by keyword similarity. Never assume missing values.")]
    pub async fn search_by_keyword(
        &self,
        Parameters(query): Parameters<KeywordRequest>,
    ) -> Result<CallToolResult, ErrorData> {
        match self
            .client
            .request(
                Method::GET,
                SEARCH_BY_KEYWORD_ENDPOINT,
                Some(serde_json::to_value(query).unwrap()),
                None,
            )
            .await
        {
            Ok(response) => Ok(CallToolResult::success(vec![
                Content::json(response).unwrap(),
            ])),
            Err(err) => Err(ErrorData::internal_error(
                format!("Error while fetching search by keyword results: {err}"),
                None,
            )),
        }
    }

    #[tool(description = "list all the available tools.")]
    pub fn list_tools(&self) -> Result<CallToolResult, ErrorData> {
        let tools: Vec<String> = self
            .tool_router
            .list_all()
            .into_iter()
            .map(|tool| tool.name.to_string())
            .collect();
        Ok(CallToolResult::success(vec![Content::text(
            tools.join("\n"),
        )]))
    }
}

#[tool_handler]
impl ServerHandler for SmartSearchMCP {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(format!(
                "This server provides the following tools: \n{:?}.",
                self.list_tools()
            )),
        }
    }
}
