use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("validation: {0}")]
    Validation(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("database: {0}")]
    Database(#[from] rusqlite::Error),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("process: {0}")]
    Process(String),
    #[error("path: {0}")]
    Path(String),
    #[error("internal: {0}")]
    Internal(String),
}

impl AppError {
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    pub fn to_command_error(&self) -> String {
        self.to_string()
    }
}

pub type AppResult<T> = Result<T, AppError>;
