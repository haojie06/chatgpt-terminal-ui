use serde::Serialize;
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
