use reqwest::{Client, Method, StatusCode, header::CONTENT_TYPE, multipart::Form};
use serde_json::{Value, from_str};

use crate::{config::BASE_URL, search::errors::RequestError};

pub mod errors;
pub mod types;

#[derive(Clone)]
pub struct SearchClient {
    client: Client,
}

impl SearchClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn request(
        &self,
        http_method: Method,
        endpoint: &str,
        params: Option<Value>,
        multipart: Option<Form>,
    ) -> Result<Value, RequestError> {
        let url = format!("{BASE_URL}/{endpoint}");
        let mut request = self.client.request(http_method, &url);

        if let Some(params) = params {
            request = request
                .header(CONTENT_TYPE, "application/json")
                .query(&params);
        }

        if let Some(form) = multipart {
            request = request
                .header(CONTENT_TYPE, "multipart/form-data")
                .multipart(form);
        }

        let response = request.send().await?;

        match response.status() {
            StatusCode::OK => {
                let json_response: Value = from_str(&response.text().await?)?;
                Ok(json_response)
            }
            StatusCode::UNPROCESSABLE_ENTITY => Err(RequestError::Api {
                status_code: response.status().into(),
                message: "Could not process request".to_owned(),
            }),
            _ => Err(RequestError::Api {
                status_code: response.status().into(),
                message: "Request failed".to_owned(),
            }),
        }
    }
}
