use crate::models::{Node, NodeContext, NodeOutput};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct HttpRequestParams {
    url: String,
    method: Option<String>,
    headers: Option<serde_json::Map<String, serde_json::Value>>,
    body: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct HttpResponse {
    status: u16,
    headers: serde_json::Map<String, serde_json::Value>,
    body: serde_json::Value,
}

/// HTTP Request node - makes HTTP requests
pub struct HttpRequestNode;

#[async_trait]
impl Node for HttpRequestNode {
    fn node_type(&self) -> &str {
        "http_request"
    }

    async fn execute(
        &self,
        _context: &NodeContext,
        parameters: &serde_json::Value,
    ) -> anyhow::Result<NodeOutput> {
        let params: HttpRequestParams = serde_json::from_value(parameters.clone())?;

        let client = reqwest::Client::new();
        let method = params.method.unwrap_or_else(|| "GET".to_string());

        let mut request = match method.to_uppercase().as_str() {
            "GET" => client.get(&params.url),
            "POST" => client.post(&params.url),
            "PUT" => client.put(&params.url),
            "DELETE" => client.delete(&params.url),
            "PATCH" => client.patch(&params.url),
            _ => return Ok(NodeOutput::error(format!("Unsupported HTTP method: {}", method))),
        };

        // Add headers
        if let Some(headers) = params.headers {
            for (key, value) in headers {
                if let Some(value_str) = value.as_str() {
                    request = request.header(&key, value_str);
                }
            }
        }

        // Add body for POST/PUT/PATCH
        if let Some(body) = params.body {
            request = request.json(&body);
        }

        // Execute request
        match request.send().await {
            Ok(response) => {
                let status = response.status().as_u16();

                // Convert headers to JSON
                let mut header_map = serde_json::Map::new();
                for (key, value) in response.headers() {
                    if let Ok(value_str) = value.to_str() {
                        header_map.insert(
                            key.to_string(),
                            serde_json::Value::String(value_str.to_string()),
                        );
                    }
                }

                // Try to parse body as JSON, fallback to text
                let body = match response.json::<serde_json::Value>().await {
                    Ok(json) => json,
                    Err(_) => {
                        // If JSON parsing fails, try to get as text
                        serde_json::Value::String("".to_string())
                    }
                };

                let result = HttpResponse {
                    status,
                    headers: header_map,
                    body,
                };

                Ok(NodeOutput::success(serde_json::to_value(result)?))
            }
            Err(e) => Ok(NodeOutput::error(format!("HTTP request failed: {}", e))),
        }
    }

    fn validate_parameters(&self, parameters: &serde_json::Value) -> anyhow::Result<()> {
        let params: HttpRequestParams = serde_json::from_value(parameters.clone())?;

        if params.url.is_empty() {
            anyhow::bail!("URL is required");
        }

        Ok(())
    }
}
