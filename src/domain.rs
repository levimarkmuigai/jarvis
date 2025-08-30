use crate::errors::JarvisError;

pub struct Prompt {
    pub raw: String,
}

impl Prompt {
    pub fn new(raw: String) -> Result<Self, JarvisError> {
        if raw.trim().is_empty() {
            return Err(JarvisError::EmptyPrompt);
        }
        Ok(Self {raw} )
    }


    pub fn to_payload(&self) -> String {
        format!("Answer this like Jarvis from Iron Man: {}", self.raw)
    }
}

pub struct Response {
    pub content: String,
}

impl Response {
    pub fn new(content: String) -> Self {
        Self { content }
    }
    pub fn value(&self) -> &str {
        &self.content
    }
}

pub trait AIClient {
    fn ask(&self, prompt: &Prompt) -> Result<Response, JarvisError>;
}

pub struct JarvisService<C: AIClient> {
    client: C,
}

impl<C: AIClient> JarvisService<C>{
    pub fn new(client: C) -> Self {
        Self { client }
    }
    pub fn ask(&self, prompt: &Prompt) -> Result<Response, JarvisError> {
        self.client.ask(prompt)
    }
}
