use core::fmt;

#[derive(Debug)]
pub enum OpenAIError {
    RequestAPIError(String),
    ParseChunkError(String),
}

impl fmt::Display for OpenAIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpenAIError::ParseChunkError(msg) => write!(f, "ParseChunkError: {}", msg),
            OpenAIError::RequestAPIError(msg) => write!(f, "RequestAPIError: {}", msg),
        }
    }
}

impl std::error::Error for OpenAIError {}

impl From<OpenAIError> for std::io::Error {
    fn from(err: OpenAIError) -> Self {
        match err {
            OpenAIError::ParseChunkError(msg) => {
                std::io::Error::new(std::io::ErrorKind::InvalidData, msg)
            }
            OpenAIError::RequestAPIError(msg) => {
                std::io::Error::new(std::io::ErrorKind::Other, msg)
            }
        }
    }
}

impl From<serde_json::Error> for OpenAIError {
    fn from(err: serde_json::Error) -> Self {
        OpenAIError::ParseChunkError(err.to_string())
    }
}

impl From<reqwest::Error> for OpenAIError {
    fn from(err: reqwest::Error) -> Self {
        OpenAIError::RequestAPIError(err.to_string())
    }
}

impl From<std::str::Utf8Error> for OpenAIError {
    fn from(err: std::str::Utf8Error) -> Self {
        OpenAIError::ParseChunkError(err.to_string())
    }
}
