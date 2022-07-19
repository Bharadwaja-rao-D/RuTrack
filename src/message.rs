use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Enroll(String, String),
    Verify(String, String),
    Chat(String),
    Ask(String, u32),
}

impl TryFrom<String> for Message {
    type Error = serde_json::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let msg: Message = serde_json::from_str(&value)?;
        return Ok(msg);
    }
}

impl TryFrom<Message> for String {
    type Error = serde_json::Error;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        let msg = serde_json::to_string(&value)?;
        return Ok(msg);
    }
}
