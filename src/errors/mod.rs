use std::collections::HashMap;

use actix_web::{ResponseError, http::StatusCode};
use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct AppError {
    message: HashMap<String, String>, 
    code: AppErrorCode
}

#[derive(Debug, PartialEq, Eq)]
pub struct AppErrorCode(i32);

impl AppErrorCode {
    pub fn message(self, message: HashMap<String, String>) -> AppError {
        AppError { message, code: self }
    }

    pub fn default(self) -> AppError {
        let message = match self {
            AppError::INVALID_INPUT => "Invalid input.",
            AppError::INVALID_CREDENTIALS => "Invalid username or password provided",
            AppError::NOT_AUTHORIZED => "Not authorized.",
            AppError::NOT_FOUND => "Item not found.",
            _ => "An unexpected error has occurred.",
        };
        let mut message_hash = HashMap::new();
        message_hash.insert("".to_string(), message.to_string());
        AppError {
            message: message_hash,
            code: self,
        }
    }
}


impl AppError {
    pub const INTERNAL_ERROR: AppErrorCode = AppErrorCode(500);
    pub const INVALID_INPUT: AppErrorCode = AppErrorCode(422);
    pub const INVALID_CREDENTIALS: AppErrorCode = AppErrorCode(401);
    pub const NOT_AUTHORIZED: AppErrorCode = AppErrorCode(401);
    pub const NOT_FOUND: AppErrorCode = AppErrorCode(404);
}

impl From<AppErrorCode> for AppError {
    fn from(error: AppErrorCode) -> Self {
        error.default()
    }
}

impl Serialize for AppErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer 
    {
        serializer.serialize_i32(self.0)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.code {
            AppError::INVALID_INPUT => StatusCode::BAD_REQUEST,
            AppError::NOT_FOUND => StatusCode::NOT_FOUND,
            AppError::INVALID_CREDENTIALS => StatusCode::UNAUTHORIZED,
            AppError::NOT_AUTHORIZED => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,

        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}