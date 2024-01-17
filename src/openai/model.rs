use serde::{Deserialize, Serialize};
#[derive(Serialize, Debug)]
pub struct CompletionRequest {
    pub model: String,
    pub messages: Vec<CompletionMessage>,
    pub stream: bool,
}
#[derive(Serialize, Debug)]
pub struct CompletionMessage {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct CompletionChunk {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub system_fingerprint: Option<String>,
    pub choices: Vec<CompletionChoice>,
}

#[derive(Deserialize, Debug)]
pub struct CompletionChoice {
    pub index: u64,
    pub delta: CompletionChoiceDelta,
    pub finish_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct CompletionChoiceDelta {
    pub content: Option<String>,
}
