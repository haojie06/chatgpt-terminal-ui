use std::collections::VecDeque;

use crate::openai::CompletionMessage;

// 记录历史会话
pub struct Conversation {
    pub window_size: usize, // 超出多少token后丢弃
    pub history: VecDeque<CompletionMessage>,
}
