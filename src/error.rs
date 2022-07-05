use thiserror::Error;

#[derive(Debug, Error)]
pub enum RutrackError {
    #[error("parse errors")]
    JsonError(#[from] serde_json::Error),
    #[error("io errors")]
    IOError(#[from] std::io::Error),
    #[error("websocket errors")]
    TungstenitError(#[from] tokio_tungstenite::tungstenite::Error),
}
