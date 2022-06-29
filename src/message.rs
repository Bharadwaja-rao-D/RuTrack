use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageType{
    Enroll(String, String),
    Verify(String, String),
    Chat(String),
    Ask(String, u32)
}
