use crate::domain::{AIClient, Prompt, Response};
use crate::errors::JarvisError;
use serde::{Deserialize, Serialize};
use reqwest::blocking::Client;
use std::env;


pub struct HttpAIClient {
    base_url: String,
    api_key: String,
}

#[derive(Serialize)]
struct HFRequest {
    model: String,
    prompt: String,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Deserialize, Debug)]
struct HFChoice {
    text: String,
}

#[derive(Deserialize, Debug)]
struct HFResponse {
    choices: Vec<HFChoice>,
}


impl HttpAIClient {
    pub fn new() -> Result<Self, JarvisError> {
        let base_url =  env::var("HUGGINGFACE_URL")
            .map_err(|_e| JarvisError::ConfigError("Missing HUGGINGFACE_URL".into()))?;

            let api_key = env::var("HUGGINGFACE_TOKEN")
            .map_err(|_e| JarvisError::ConfigError("Missing HUGGINGFACE_TOKEN".into()))?;

        Ok(Self { base_url, api_key})
    }
}

impl AIClient for HttpAIClient {
    fn ask(&self, prompt: &Prompt) -> Result<Response, JarvisError> {
        let request_body = HFRequest {
            model: "meta-llama/Meta-Llama-3.1-8B".to_string(),
            prompt: prompt.to_payload(),
            max_tokens: 200,
            temperature: 0.7,
        };

        let client = Client::new();

        let resp = client
            .post(&self.base_url)
            .bearer_auth(&self.api_key)
            .json(&request_body)
            .send()
            .map_err(|e| JarvisError::NetworkError(e.to_string()))?;

        let raw_text = resp
            .text()
            .map_err(|e| JarvisError::ConfigError(e.to_string()))?;

        let hf_response: HFResponse = serde_json::from_str(&raw_text)
            .map_err(|e| JarvisError::InvalidResponse(e.to_string()))?;

        if let Some(first) = hf_response.choices.into_iter().next() {
            Ok(Response::new(first.text))
        } else {
            Err(JarvisError::InvalidResponse("Empty choices".into()))
        }
    }
}

