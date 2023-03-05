use std::env::VarError;
use std::fmt;
use std::fmt::{Display, Formatter};

pub enum RestError {
    Unknown,
    NotFound,
    AuthFailure,
}

impl From<hyper::Error> for RestError {
    fn from(_value: hyper::Error) -> Self {
        RestError::Unknown
    }
}

impl From<VarError> for RestError {
    fn from(_value: VarError) -> Self {
        RestError::Unknown
    }
}

pub enum RestSchema {
    Http,
    Https,
}

impl From<String> for RestSchema {
    fn from(value: String) -> Self {
        match value.to_uppercase().as_str() {
            "HTTP" => RestSchema::Http,
            "HTTPS" => RestSchema::Https,
            _ => RestSchema::Http,
        }
    }
}

impl Display for RestSchema {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RestSchema::Http => write!(f, "http"),
            RestSchema::Https => write!(f, "https"),
        }
    }
}
