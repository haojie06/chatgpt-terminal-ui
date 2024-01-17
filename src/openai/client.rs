use std::collections::VecDeque;

use super::{
    error::OpenAIError,
    model::{CompletionChunk, CompletionRequest},
};
use futures_util::{Stream, StreamExt};
use reqwest::{header::CONTENT_TYPE, Client};
pub struct OpenAIClientConfig {
    pub base_url: Option<String>,
    pub api_key: String,
}

pub struct OpenAIClient {
    pub base_url: String,
    pub api_key: String,
}

pub type StreamResult = Result<bytes::Bytes, reqwest::Error>;
pub struct CompletionChunkReader<T>
where
    T: Stream<Item = StreamResult> + Unpin,
{
    stream: T,
    str_datas: VecDeque<String>,
    done: bool,
}

impl<T> CompletionChunkReader<T>
where
    T: Stream<Item = StreamResult> + Unpin,
{
    pub fn new(stream: T) -> Self {
        Self {
            stream,
            str_datas: VecDeque::new(),
            done: false,
        }
    }

    // get data from stream
    async fn get_data_from_stream(&mut self) -> Result<(), OpenAIError> {
        if let Some(chunk) = self.stream.next().await {
            let chunk = chunk?;
            let chunk = std::str::from_utf8(&chunk)?;
            for chunk_p in chunk.split("\n\n") {
                if let Some(chunk_str) = chunk_p.strip_prefix("data: ") {
                    if chunk_str == "[DONE]" {
                        self.done = true;
                        break;
                    }
                    self.str_datas.push_back(chunk_str.to_string());
                }
            }
        }
        Ok(())
    }

    pub async fn next_chunk(&mut self) -> Result<Option<CompletionChunk>, OpenAIError> {
        loop {
            if self.str_datas.is_empty() {
                if self.done {
                    println!("done");
                    return Ok(None);
                }
                self.get_data_from_stream().await?;
            }
            // 读取缓存的data, 返回第一个能够成功解析的
            while let Some(data_str) = self.str_datas.pop_front() {
                match serde_json::from_str::<CompletionChunk>(&data_str) {
                    Ok(chunk) => return Ok(Some(chunk)),
                    Err(_err) => {
                        // 如果不能够成功解析，就继续读取下一个
                        // eprintln!("parse chunk error: {} {}", err, data_str);
                        continue;
                    }
                }
            }
        }
    }
}

// impl<T> Stream for CompletionChunkReader<T>
// where
//     T: Stream<Item = Result<bytes::Bytes, reqwest::Error>> + Unpin,
// {
//     type Item = Result<CompletionChunk, OpenAIError>;
//     fn poll_next(
//         self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Option<Self::Item>> {
//     }
// }

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
    ) -> Result<CompletionChunkReader<impl Stream<Item = StreamResult>>, OpenAIError> {
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
        Ok(CompletionChunkReader::new(resp.bytes_stream()))
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
