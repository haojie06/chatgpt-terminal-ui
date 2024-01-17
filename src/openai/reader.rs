use std::collections::VecDeque;

use futures_util::{Stream, StreamExt};

use super::{CompletionChunk, OpenAIError, StreamResult};

pub struct CompletionStreamReader<T: Stream<Item = StreamResult> + Unpin> {
    stream: T,
    str_datas: VecDeque<String>,
    done: bool,
}

impl<T: Stream<Item = StreamResult> + Unpin> CompletionStreamReader<T> {
    pub fn new(stream: T) -> Self {
        Self {
            stream,
            str_datas: VecDeque::new(),
            done: false,
        }
    }

    /// get datas split by "\n\n" from event stream
    async fn get_datas_from_stream(&mut self) -> Result<(), OpenAIError> {
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

    /// convert to content reader
    pub fn to_content_reader(self) -> CompletionContentReader<T> {
        CompletionContentReader::new(self)
    }

    /// get next chunk, until get a valid chunk (can be deserialize to CompletionChunk)
    pub async fn next_chunk(&mut self) -> Result<Option<CompletionChunk>, OpenAIError> {
        loop {
            if self.str_datas.is_empty() {
                if self.done {
                    println!("done");
                    return Ok(None);
                }
                self.get_datas_from_stream().await?;
            }
            // 读取缓存的data, 返回第一个能够成功解析的
            while let Some(data_str) = self.str_datas.pop_front() {
                match serde_json::from_str::<CompletionChunk>(&data_str) {
                    Ok(chunk) => return Ok(Some(chunk)),
                    Err(_err) => {
                        // if parse error, just ignore this chunk
                        // eprintln!("invalid chunk: {}", data_str);
                        continue;
                    }
                }
            }
        }
    }
}

pub struct CompletionContentReader<T>
where
    T: Stream<Item = StreamResult> + Unpin,
{
    chunk_reader: CompletionStreamReader<T>,
    content: String,
}

impl<T> CompletionContentReader<T>
where
    T: Stream<Item = StreamResult> + Unpin,
{
    pub fn new(chunk_reader: CompletionStreamReader<T>) -> Self {
        Self {
            chunk_reader,
            content: String::new(),
        }
    }

    pub async fn next_content(&mut self) -> Result<Option<String>, OpenAIError> {
        match self.chunk_reader.next_chunk().await? {
            Some(chunk) => {
                if chunk.choices.is_empty() {
                    return Ok(Some("".to_string()));
                }
                if let Some(ref delta_content) = chunk.choices.get(0).unwrap().delta.content {
                    self.content.push_str(delta_content);
                }
                Ok(Some(self.content.clone()))
            }
            None => Ok(None),
        }
    }
}
