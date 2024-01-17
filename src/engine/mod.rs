mod conversation;
use std::error::Error;

use futures_util::Stream;

use crate::openai::{self, CompletionContentReader, OpenAIClient, StreamResult};

pub struct CompletionClient {
    openai_client: OpenAIClient,
}

impl CompletionClient {
    pub fn new(openai_cfg: openai::OpenAIClientConfig) -> Self {
        Self {
            openai_client: OpenAIClient::new(openai_cfg),
        }
    }

    /// ask a question, return a stream of answers
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

    /// chat with openai, return a stream of answers, maintain the context
    pub async fn chat(
        &self,
        conversation_id: Option<String>,
        prompt: String,
        completion_model: openai::CompletionModel,
    ) -> Result<String, &dyn Error> {
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
