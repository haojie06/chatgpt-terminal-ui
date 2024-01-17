use super::{error::OpenAIError, model::CompletionRequest};
use super::{CompletionStreamReader, StreamResult};
use futures_util::Stream;
use reqwest::{header::CONTENT_TYPE, Client};

pub struct OpenAIClientConfig {
    pub base_url: Option<String>,
    pub api_key: String,
}

pub struct OpenAIClient {
    pub base_url: String,
    pub api_key: String,
}

impl OpenAIClient {
    pub fn new(cfg: OpenAIClientConfig) -> Self {
        Self {
            base_url: cfg
                .base_url
                .unwrap_or("https://api.openai.com/v1".to_string()),
            api_key: cfg.api_key,
        }
    }

    pub async fn chat_completion_stream(
        &self,
        completion_request: CompletionRequest,
    ) -> Result<CompletionStreamReader<impl Stream<Item = StreamResult>>, OpenAIError> {
        let req_url = format!("{}/chat/completions", self.base_url);
        let client = Client::new();
        let body = serde_json::to_string(&completion_request).unwrap();
        let resp = client
            .post(req_url)
            .header(CONTENT_TYPE, "application/json")
            .bearer_auth(&self.api_key)
            .body(body)
            .send()
            .await
            .unwrap();

        println!("status: {}", resp.status());
        Ok(CompletionStreamReader::new(resp.bytes_stream()))
    }
}

#[cfg(test)]
mod tests {
    use crate::openai::model::{CompletionMessage, CompletionModel};

    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_completion_stream() {
        let openai_key = env::var("OPENAI_KEY").expect("need OPENAI_KEY");
        let base_url = env::var("OPENAI_BASE_URL").ok();
        let client = OpenAIClient::new(OpenAIClientConfig {
            base_url: base_url,
            api_key: openai_key,
        });
        let completion_request = CompletionRequest {
            model: CompletionModel::GPT3_5Turbo,
            messages: vec![
                CompletionMessage::default_system_message(),
                CompletionMessage::new_user_message("hello world".to_string()),
            ],
            stream: true,
        };

        let mut chunk_reader = client
            .chat_completion_stream(completion_request)
            .await
            .unwrap();
        while let Some(chunk) = chunk_reader.next_chunk().await.unwrap() {
            println!("chunk: {:?}", chunk);
        }
    }
}
