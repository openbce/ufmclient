use std::env::VarError;
use std::fmt;
use std::fmt::{Display, Formatter};

pub enum RestError {
    Unknown { msg: String },
    NotFound { msg: String },
    AuthFailure { msg: String },
    InvalidConfig { msg: String },
}

impl From<hyper::Error> for RestError {
    fn from(value: hyper::Error) -> Self {
        if value.is_user() {
            return RestError::AuthFailure {
                msg: value.message().to_string(),
            };
        }

        RestError::Unknown {
            msg: value.message().to_string(),
        }
    }
}

impl From<VarError> for RestError {
    fn from(value: VarError) -> Self {
        RestError::InvalidConfig {
            msg: value.to_string(),
        }
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
