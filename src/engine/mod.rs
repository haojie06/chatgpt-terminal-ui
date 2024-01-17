mod conversation;
use std::error::Error;

use futures_util::Stream;

use crate::openai::{self, CompletionContentReader, OpenAIClient, StreamResult};

pub struct CompletionClient {
    openai_client: OpenAIClient,
}

// 程序对话client，负责和openai交互以及上下文的处理
impl CompletionClient {
    pub fn new(openai_cfg: openai::OpenAIClientConfig) -> Self {
        Self {
            openai_client: OpenAIClient::new(openai_cfg),
        }
    }

    // 一轮的对话，不记录上下文
    pub async fn ask(
        &self,
        prompt: String,
        completion_model: openai::CompletionModel,
    ) -> Result<CompletionContentReader<impl Stream<Item = StreamResult> + Unpin>, Box<dyn Error>>
    {
        let completion_request = openai::CompletionRequest {
            model: completion_model,
            messages: vec![
                openai::CompletionMessage::default_system_message(),
                openai::CompletionMessage::new_user_message(prompt),
            ],
            stream: true,
        };
        let chunk_reader = self
            .openai_client
            .chat_completion_stream(completion_request)
            .await?;
        Ok(chunk_reader.to_content_reader())
    }

    // 记录上下文对话
    pub async fn chat(&self, prompt: String) -> Result<String, &dyn Error> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_ask() {
        let openai_key = env::var("OPENAI_KEY").expect("need OPENAI_KEY");
        let base_url = env::var("OPENAI_BASE_URL").ok();
        let client = CompletionClient::new(openai::OpenAIClientConfig {
            base_url: base_url,
            api_key: openai_key,
        });
        let mut content_reader = client
            .ask(
                "hello, who are you?".to_string(),
                openai::CompletionModel::GPT3_5Turbo,
            )
            .await
            .unwrap();
        while let Some(content) = content_reader.next_content().await.unwrap() {
            println!("content: {}", content);
        }
    }
}
