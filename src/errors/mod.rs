use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error as DieselError;
use serde::Deserialize;
use serde_json::json;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct CustomError {
    pub status_code: u16,
    pub error_message: String,
}

impl CustomError {
    pub fn new(status_code: u16, error_message: String) -> CustomError {
        CustomError { 
            status_code: status_code,
            error_message: error_message
        }
    }
}

/// Implement Display trait for CustomError.
impl fmt::Display for CustomError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&self.error_message.as_str())
    }
}

/// Implement conversion from DieselError for CustomError.
impl From<DieselError> for CustomError {
    fn from(error: DieselError) -> CustomError {
        match error {
            DieselError::DatabaseError(_, err) => CustomError::new(409, err.message().to_string()),
            DieselError::NotFound => {
                CustomError::new(404, "Recipe record not found.".to_string())
            },
            err => CustomError::new(500, format!("Unknown Diesel error: {}", err),)
        }
    }
}

/// Implement ResponseError trait for CustomError.
impl ResponseError for CustomError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let error_message = match status_code.as_u16() < 500 {
            true => self.error_message.clone(),
            false => "Internal server error.".to_string(),
        };

        HttpResponse::build(status_code).json(json!({"message": error_message}))
    }
}