use bytes::Bytes;
use reqwest::StatusCode;
use std::fmt::{Debug, Formatter};
use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
    /// Error occurred while using the `reqwest` library.
    ReqwestError(reqwest::Error),

    /// An API request returned with a failed status code.
    RequestFailed {
        bytes: Bytes,
        status_code: StatusCode,
    },

    /// Error occurred while using the `serde` library.
    SerdeError(serde_json::Error),
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReqwestError(e) => std::fmt::Display::fmt(&e, f),
            Self::RequestFailed { bytes, status_code } => {
                let text = String::from_utf8_lossy(bytes);
                write!(f, "{status_code}: {text}")
            }
            Self::SerdeError(e) => write!(f, "{e}"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::ReqwestError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeError(e)
    }
}
