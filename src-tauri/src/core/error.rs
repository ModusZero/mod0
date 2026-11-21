use serde::{Serialize, Serializer};

/// Error unificado de la aplicación. Se serializa como texto plano para que el
/// frontend reciba un `string` legible en el `catch` de cada `invoke`.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("json: {0}")]
    Json(#[from] serde_json::Error),

    #[error("tauri: {0}")]
    Tauri(#[from] tauri::Error),

    #[error("git: {0}")]
    Git(String),

    #[error("sesión no encontrada: {0}")]
    SessionNotFound(String),

    #[error("{0}")]
    Msg(String),
}

impl AppError {
    pub fn msg(message: impl Into<String>) -> Self {
        Self::Msg(message.into())
    }
}

impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
