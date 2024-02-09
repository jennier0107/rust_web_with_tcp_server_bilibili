use std::fmt::{self, Formatter};

/// http 请求版本
#[derive(Debug, PartialEq)]
pub enum HttpVersion {
    V1_1,
    V2_0,
    Uninitialized,
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            HttpVersion::V1_1 | HttpVersion::Uninitialized => write!(f, "HTTP/1.1"),
            HttpVersion::V2_0 => write!(f, "HTTP/2.0"),
        }
    }
}

impl From<&str> for HttpVersion {
    fn from(value: &str) -> Self {
        match value {
            "HTTP/1.1" => Self::V1_1,
            "HTTP/2.0" => Self::V2_0,
            _ => Self::Uninitialized,
        }
    }
}