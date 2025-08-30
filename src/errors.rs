use thiserror::Error;

#[derive(Error, Debug)]
pub enum JarvisError {

    #[error("Prompt cannot be empty")]
    EmptyPrompt,

    #[error("Configutation error: {0}")]
    ConfigError(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Invalid response from AI: {0}")]
    InvalidResponse(String),
}
