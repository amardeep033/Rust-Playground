use serde_json::json;

use crate::err::AppError;

//-------------------------------------------------------------------------------------------------------------

pub trait ErrorRenderer {
    fn render(&self, error: &AppError) -> String;
}

pub enum ErrorMode {
    Detailed,
    Minimal,
}

//strategy pattern
pub fn create_error_renderer(mode: ErrorMode) -> Box<dyn ErrorRenderer> {
    match mode {
        ErrorMode::Detailed => Box::new(DetailedErrorRenderer),
        ErrorMode::Minimal => Box::new(MinimalErrorRenderer),
    }
}

//-------------------------------------------------------------------------------------------------------------

pub struct DetailedErrorRenderer;

impl ErrorRenderer for DetailedErrorRenderer {
    fn render(&self, error: &AppError) -> String {
        json!({
            "success": false,
            "error": {
                "code": error.code(),
                "message": error.readable_message(),
                "category": error.category()
            }
        })
        .to_string()
    }
}

//-------------------------------------------------------------------------------------------------------------

pub struct MinimalErrorRenderer;

impl ErrorRenderer for MinimalErrorRenderer {
    fn render(&self, _error: &AppError) -> String {
        json!({
            "success": false,
            "error": "Request failed"
        })
        .to_string()
    }
}
