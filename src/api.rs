use crate::domain::{AIClient, Prompt, Response};
use crate::errors::JarvisError;
use serde::{Deserialize, Serialize};
use serde_json;
use reqwest::blocking::Client;
use std::env;


struct HttpAIClient {
    base_url: String,
    api_key: String,
}

#[derive(Serialize)]
struct HFRequest {
    inputs: String,
}

#[derive(Deserialize)]
struct HFResponse {
    generated_text: String,
}

impl HttpAIClient {
    fn new() -> Result<Self, JarvisError> {
        let base_url =  env::var("HUGGINGFACE_URL")
            .map_err(|e| JarvisError::ConfigError("Missing HUGGINGFACE_URL".into()))?;

            let api_key = env::var("HUGGINGFACE_TOKEN")
            .map_err(|e| JarvisError::ConfigError("Missing HUGGINGFACE_TOKEN".into()))?;

        Ok(Self { base_url, api_key})
    }
}

impl AIClient for HttpAIClient {
    fn ask(&self, prompt: &Prompt) -> Result<Response, JarvisError> {

        let request_body = HFRequest {
            inputs: prompt.to_payload(),
        };

        let client = Client::new();

        let resp = client
            .post(&self.base_url)
            .bearer_auth(&self.api_key)
            .json(&request_body)
            .send()
            .map_err(|e| JarvisError::NetworkError(e.to_string()))?;

        let hf_response: Vec<HFResponse> = resp
            .json()
            .map_err(|e| JarvisError::InvalidResponse(e.to_string()))?;

        if let Some(first) = hf_response.into_iter().next() {
            Ok(Response::new(first.generated_text))
        } else {
            Err(JarvisError::InvalidResponse("Empty response from Hugging face".to_string(),
            ))
        }
    }
}
