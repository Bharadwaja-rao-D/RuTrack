use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Enroll(String, String),
    Verify(String, String),
    Chat(String),
    Ask(String, u32),
}
