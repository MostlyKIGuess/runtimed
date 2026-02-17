use thiserror::Error;

#[derive(Error, Debug)]
pub enum YSyncError {
    #[error("Failed to convert notebook to Y.Doc: {0}")]
    ConversionError(String),

    #[error("Invalid cell type: {0}")]
    InvalidCellType(String),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Y.Doc transaction error: {0}")]
    TransactionError(String),

    #[error("Protocol error: {0}")]
    ProtocolError(String),
}

pub type Result<T> = std::result::Result<T, YSyncError>;
