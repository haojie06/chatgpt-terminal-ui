use core::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum CompletionModel {
    #[serde(rename = "gpt-3.5-turbo")]
    GPT3_5Turbo,
    #[serde(rename = "gpt-4-turbo")]
    GPT4Turbo,
}

impl fmt::Display for CompletionModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompletionModel::GPT3_5Turbo => write!(f, "gpt-3.5-turbo"),
            CompletionModel::GPT4Turbo => write!(f, "gpt-4-turbo"),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub enum CompletionRole {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
}

impl fmt::Display for CompletionRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CompletionRole::System => write!(f, "system"),
            CompletionRole::User => write!(f, "user"),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct CompletionRequest {
    pub model: CompletionModel,
    pub messages: Vec<CompletionMessage>,
    pub stream: bool,
}
#[derive(Serialize, Debug)]
pub struct CompletionMessage {
    pub role: CompletionRole,
    pub content: String,
}

impl CompletionMessage {
    pub fn default_system_message() -> Self {
        Self {
            role: CompletionRole::System,
            content: String::from("You are a helpful assistant."),
        }
    }

    pub fn new_user_message(content: String) -> Self {
        Self {
            role: CompletionRole::User,
            content,
        }
    }
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
