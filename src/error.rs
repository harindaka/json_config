extern crate serde_json;

use std::error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum JsonConfigError{
    ConfigDefinition(String),
    Io(io::Error),
    SerdeJson(serde_json::error::Error)
}

impl From<String> for JsonConfigError {
    fn from(err: String) -> JsonConfigError {
        JsonConfigError::ConfigDefinition(err)
    }
}

impl From<io::Error> for JsonConfigError {
    fn from(err: io::Error) -> JsonConfigError {
        JsonConfigError::Io(err)
    }
}

impl From<serde_json::error::Error> for JsonConfigError {
    fn from(err: serde_json::error::Error) -> JsonConfigError {
        JsonConfigError::SerdeJson(err)
    }
}

impl fmt::Display for JsonConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            JsonConfigError::ConfigDefinition(ref err) => write!(f, "json_config Configuration Definition Error: {}", err),
            JsonConfigError::Io(ref err) => write!(f, "I/O error: {}", err),
            JsonConfigError::SerdeJson(ref err) => write!(f, "serde_json error: {}", err),
        }
    }
}

impl error::Error for JsonConfigError {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            JsonConfigError::ConfigDefinition(ref err) => err,
            JsonConfigError::Io(ref err) => error::Error::description(err),
            JsonConfigError::SerdeJson(ref err) => error::Error::description(err),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            JsonConfigError::ConfigDefinition(ref err) => None,
            JsonConfigError::Io(ref err) => Some(err),
            JsonConfigError::SerdeJson(ref err) => Some(err),
        }
    }
}