use std::{
    error::Error,
    fmt::{self, Display},
};

// CUSTOM ERROR STRUCTS + ENUMS
// -----------------------------------------------

/// A comprehensive set of errors that might occur in the Kalshi module.
///
/// This enum encompasses various types of errors, including HTTP request errors,
/// user input errors, and internal errors. It provides a unified error type for
/// the entire Kalshi module.
///
/// Represents various errors that can occur when interacting with the Kalshi API.
#[derive(Debug)]
pub enum KalshiError {
    /// Errors that occur during HTTP requests. This includes connectivity issues,
    /// response serialization problems, and HTTP status errors.
    RequestError(RequestError),
    /// Errors caused by incorrect or invalid user input.
    UserInputError(String),
    /// Errors representing unexpected internal issues or situations that are not supposed to happen.
    InternalError(String),
    // TODO: add error type specifically for joining threads together.
}

impl Display for KalshiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KalshiError::RequestError(e) => write!(f, "HTTP Error: {}", e),
            KalshiError::UserInputError(e) => write!(f, "User Input Error: {}", e),
            KalshiError::InternalError(e) => write!(f, "INTERNAL ERROR, PLEASE EMAIL DEVELOPER OR MAKE A NEW ISSUE ON THE CRATE'S REPOSITORY: https://github.com/dpeachpeach/kalshi-rust. Specific Error: {}", e)
        }
    }
}

impl Error for KalshiError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            KalshiError::RequestError(e) => Some(e),
            KalshiError::UserInputError(_) => None,
            KalshiError::InternalError(_) => None,
        }
    }
}

impl From<reqwest::Error> for KalshiError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_decode() {
            KalshiError::RequestError(RequestError::SerializationError(err))
        } else if err.is_status() {
            if let Some(status) = err.status() {
                if status.is_client_error() {
                    KalshiError::RequestError(RequestError::ClientError(err))
                } else if status.is_server_error() {
                    KalshiError::RequestError(RequestError::ServerError(err))
                } else {
                    KalshiError::RequestError(RequestError::ServerError(err))
                }
            } else {
                KalshiError::RequestError(RequestError::ServerError(err))
            }
        } else if err.is_body() || err.is_timeout() {
            KalshiError::RequestError(RequestError::ServerError(err))
        } else {
            KalshiError::RequestError(RequestError::ServerError(err))
        }
    }
}

/// Represents errors specific to HTTP requests within the Kalshi API client.
#[derive(Debug)]
pub enum RequestError {
    /// Errors occurring during serialization or deserialization of request or response data.
    SerializationError(reqwest::Error),
    /// Errors representing client-side request issues, such as bad requests or unauthorized access.
    ClientError(reqwest::Error),
    /// Errors indicating server-side issues, like internal server errors or service unavailability.
    ServerError(reqwest::Error),
    /// Errors occurring during URL parsing.
    UrlParseError(url::ParseError),
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestError::SerializationError(e) => write!(f, "Serialization Error. You connected successfully but either: Your inputs to a request were incorrect or the exchange is closed! {}", e),
            RequestError::ClientError(e) => {
                if let Some(status) = e.status() {
                    write!(f, "Client Request Error, Status code: {}", status)
                } else {
                    write!(f, "Client Request Error: {}", e)
                }
            },
            RequestError::ServerError(e) => {
                if let Some(status) = e.status() {
                    write!(f, "Server Request Error, Status code: {}", status)
                } else {
                    write!(f, "Server Request Error: {}", e)
                }
            },
            RequestError::UrlParseError(e) => write!(f, "URL Parse Error: {}", e),
        }
    }
}

impl Error for RequestError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            RequestError::ClientError(e) => Some(e),
            RequestError::ServerError(e) => Some(e),
            RequestError::SerializationError(e) => Some(e),
            RequestError::UrlParseError(e) => Some(e),
        }
    }
}
