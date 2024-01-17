mod conversation;
use std::error::Error;

use crate::openai::{self, CompletionChunkReader, OpenAIClient, StreamResult};
use futures_util::stream::Stream;

pub struct CompletionIterator<T>
where
    T: Stream<Item = StreamResult> + Unpin,
{
    chunk_reader: CompletionChunkReader<T>,
    content: String,
}

impl<T> CompletionIterator<T>
where
    T: Stream<Item = StreamResult> + Unpin,
{
    pub fn new(chunk_reader: CompletionChunkReader<T>) -> Self {
        Self {
            chunk_reader,
            content: String::new(),
        }
    }

    pub async fn next_completion(&mut self) -> Result<String, Box<dyn Error>> {
        unimplemented!()
        // if let Some(chunk) = self.chunk_reader.next_chunk().await.unwrap() {
        //     if let Some(ref delta_content) = chunk.choices.get(0).unwrap().delta.content {
        //         self.content.push_str(delta_content);
        //         println!("answer: {}", self.content);
        //     }
        // } else {
        //     return None;
        // }
    }
}

pub struct Client {
    openai_client: OpenAIClient,
}

// 程序对话client，负责和openai交互以及上下文的处理
impl Client {
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
    ) -> Result<String, Box<dyn Error>> {
        let completion_request = openai::CompletionRequest {
            model: completion_model,
            messages: vec![
                openai::CompletionMessage::default_system_message(),
                openai::CompletionMessage::new_user_message(prompt),
            ],
            stream: true,
        };
        let mut chunk_reader = self
            .openai_client
            .chat_completion_stream(completion_request)
            .await
            .unwrap();
        let mut answer = String::new();
        while let Some(chunk) = chunk_reader.next_chunk().await.unwrap() {
            if chunk.choices.is_empty() {
                continue;
            }
            if let Some(ref delta_content) = chunk.choices.get(0).unwrap().delta.content {
                answer.push_str(delta_content);
                println!("answer: {}", answer);
            }
        }
        answer
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
        let client = Client::new(openai::OpenAIClientConfig {
            base_url: base_url,
            api_key: openai_key,
        });
        client
            .ask(
                "hello, who are you?".to_string(),
                openai::CompletionModel::GPT3_5Turbo,
            )
            .await;
    }
}
