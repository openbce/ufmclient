use std::env::VarError;
use std::fmt;
use std::fmt::{Display, Formatter};

pub enum RestError {
    Unknown,
    NotFound,
    AuthFailure,
}

impl From<hyper::Error> for RestError {
    fn from(value: hyper::Error) -> Self {
        match value {
            _ => RestError::Unknown,
        }
    }
}

impl From<VarError> for RestError {
    fn from(value: VarError) -> Self {
        match value {
            _ => RestError::Unknown,
        }
    }
}

pub enum RestSchema {
    HTTP,
    HTTPS,
}

impl From<String> for RestSchema {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "HTTP" => RestSchema::HTTP,
            "HTTPS" => RestSchema::HTTPS,
            _ => RestSchema::HTTP,
        }
    }
}

impl Display for RestSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RestSchema::HTTP => write!(f, "http"),
            RestSchema::HTTPS => write!(f, "https"),
        }
    }
}
