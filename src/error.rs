use axum::{
    http::StatusCode,
    response::{IntoResponse, Response as AxumResponse},
    Json,
};
use http::header::{InvalidHeaderName, InvalidHeaderValue};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Debug, Display, Formatter},
    net::AddrParseError, num::{ParseIntError, ParseFloatError},
};
use tracing::debug;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TABError {
    Unknown(String),
    Invalid(String),
    Connection(String),
}

impl Default for TABError {
    fn default() -> Self {
        Self::Unknown(String::from("unknown error occured"))
    }
}

impl TABError {
    pub fn get_message(&self) -> Json<String> {
        match self {
            Self::Unknown(s) => Json(s.clone()),
            Self::Invalid(s) => Json(s.clone()),
            Self::Connection(s) => Json(s.clone()),
        }
    }

    pub fn get_message_string(&self) -> String {
        match self {
            Self::Unknown(s) => s.clone(),
            Self::Invalid(s) => s.clone(),
            Self::Connection(s) => s.clone(),
        }
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Invalid(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Connection(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Display for TABError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown(s) => {
                debug!("TABError Unknown: {}", s);
                write!(f, "TABError Unknown: {}", s)
            }
            Self::Invalid(s) => {
                debug!("TABError Invalid: {}", s);
                write!(f, "TABError Invalid: {}", s)
            }
            Self::Connection(s) => {
                debug!("TABError Connection: {}", s);
                write!(f, "TABError Connection: {}", s)
            }
        }
    }
}

impl std::error::Error for TABError {}

impl From<AddrParseError> for TABError {
    fn from(value: AddrParseError) -> Self {
        Self::Invalid(format!("address parse error: {}", value))
    }
}

impl From<InvalidHeaderName> for TABError {
    fn from(value: InvalidHeaderName) -> Self {
        Self::Invalid(format!("invalid header name: {}", value))
    }
}

impl From<InvalidHeaderValue> for TABError {
    fn from(value: InvalidHeaderValue) -> Self {
        Self::Invalid(format!("invalid header value: {}", value))
    }
}

impl From<std::io::Error> for TABError {
    fn from(value: std::io::Error) -> Self {
        Self::Connection(format!("std io error: {}", value))
    }
}

impl From<serde_json::Error> for TABError {
    fn from(value: serde_json::Error) -> Self {
        Self::Invalid(format!("serde_json error: {}", value))
    }
}

impl From<std::string::FromUtf8Error> for TABError {
    fn from(value: std::string::FromUtf8Error) -> Self {
        Self::Invalid(format!("invalid string error: {}", value))
    }
}

impl tabresp::ResponseError for TABError {
    fn status(&self) -> StatusCode {
        self.status_code()
    }

    fn message(&self) -> String {
        self.get_message_string()
    }
}

impl IntoResponse for TABError {
    fn into_response(self) -> AxumResponse {
        let r = tabresp::ErrorResponse::new(self);
        r.into_response()
    }
}

impl From<ParseIntError> for TABError {
    fn from(value: ParseIntError) -> Self {
        TABError::Invalid("integer invalid".to_string())
    }
}

impl From<ParseFloatError> for TABError {
    fn from(value: ParseFloatError) -> Self {
        TABError::Invalid("invalid float".to_string())
    }
}
