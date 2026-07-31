


//-------------------------------------------------------------------------------------------

// If not using `thiserror` with `#[from]`, uncomment the manual impls below
// and replace `#[derive(Debug, thiserror::Error)]` with `#[derive(Debug)]`.
// So thiserror is saving us from writing these external-trait impls manually:
// - std::fmt::Display for AppError
// - std::error::Error for AppError
// - std::convert::From<serde_json::Error> for AppError

#[derive(Debug)]
pub enum AppError {
    Json(serde_json::Error),
    Validation { code: &'static str, message: String },
    UnsupportedCommand { command: String },
}

//implementing external trait for internal type
impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Json(error) => write!(f, "JSON error: {error}"),
            AppError::Validation { message, .. } => {
                write!(f, "Validation error: {message}")
            }
            AppError::UnsupportedCommand { command } => {
                write!(f, "Unsupported command: {command}")
            }
        }
    }
}

impl std::error::Error for AppError {}

impl AppError {
    pub fn json(error: serde_json::Error) -> Self {
        AppError::Json(error)
    }
}

//-------------------------------------------------------------------------------------------

// #[derive(Debug, thiserror::Error)]
// pub enum AppError {
//     #[error("JSON error: {0}")]
//     Json(#[from] serde_json::Error),

//     #[error("Validation error: {message}")]
//     Validation { code: &'static str, message: String },

//     #[error("Unsupported command: {command}")]
//     UnsupportedCommand { command: String },
// }

//-------------------------------------------------------------------------------------------

impl AppError {
    pub fn code(&self) -> &'static str {
        match self {
            AppError::Json(error) => error.error_code(),
            AppError::Validation { code, .. } => code,
            AppError::UnsupportedCommand { .. } => "UNSUPPORTED_COMMAND",
        }
    }

    pub fn category(&self) -> &'static str {
        match self {
            AppError::Json(_) => "json",
            AppError::Validation { .. } => "validation",
            AppError::UnsupportedCommand { .. } => "command",
        }
    }

    pub fn readable_message(&self) -> String {
        match self {
            AppError::Json(error) => error.readable_message(),
            _ => self.to_string(),
        }
    }
}

//-------------------------------------------------------------------------------------------

pub trait JsonErrorDetails {
    fn error_code(&self) -> &'static str;
    fn readable_message(&self) -> String;
}

//implementing internal trait for external type
impl JsonErrorDetails for serde_json::Error {
    fn error_code(&self) -> &'static str {
        if self.is_data() {
            "INVALID_JSON_FIELD_TYPE"
        } else if self.is_eof() || self.is_syntax() {
            "MALFORMED_JSON"
        } else {
            "JSON_ERROR"
        }
    }

    fn readable_message(&self) -> String {
        self.to_string()
    }
}
