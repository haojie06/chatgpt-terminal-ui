use super::model::{CompletionMessage, CompletionRequest};
use futures_util::StreamExt;
use reqwest::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Client,
};
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

    pub async fn completion_stream(&self, prompt: String) {
        let req_url = format!("{}/chat/completions", self.base_url);
        let client = Client::new();
        let completion_request = CompletionRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![
                CompletionMessage {
                    role: "system".to_string(),
                    content: "You are a helpful assistant.".to_string(),
                },
                CompletionMessage {
                    role: "user".to_string(),
                    content: prompt,
                },
            ],
            stream: true,
        };
        let body = serde_json::to_string(&completion_request).unwrap();
        let resp = client
            .post(req_url)
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(CONTENT_TYPE, "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        println!("status: {}", resp.status());
        let mut stream = resp.bytes_stream();
        while let Some(item) = stream.next().await {
            println!("chunk: {:?}", item);
        }
    }
}

#[cfg(test)]
mod tests {
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
        client.completion_stream("hello world".to_string()).await;
    }
}
