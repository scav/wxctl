use std::error::Error;
use std::fmt::{self};

#[derive(Debug)]
pub enum WxError {
    Network(ureq::Error),
    Response(ureq::Error),
    EmptyResult,
    MissingName,
    MissingCountry,
}

impl fmt::Display for WxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WxError::Network(_) => write!(f, "Network error"),
            WxError::Response(_) => write!(f, "Invalid response"),
            WxError::EmptyResult => write!(f, "Empty response"),
            WxError::MissingName => write!(f, "Location name required"),
            WxError::MissingCountry => write!(f, "Country required"),
        }
    }
}

impl Error for WxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            WxError::Network(err) | WxError::Response(err) => Some(err),
            _ => None,
        }
    }
}

impl From<ureq::Error> for WxError {
    fn from(err: ureq::Error) -> Self {
        WxError::Network(err)
    }
}
