extern crate serde_json;

use std::error;
use std::fmt;
use std::io;
use std::env;

#[derive(Debug)]
pub enum JsonConfigError{
    ConfigDefinition(String),
    //BundleNotFound(String),
    EnvVar(env::VarError),
    Io(io::Error),
    SerdeJson(serde_json::error::Error)    
}

impl From<String> for JsonConfigError {
    fn from(err: String) -> JsonConfigError {
        JsonConfigError::ConfigDefinition(err)
    }
}

// impl From<String> for JsonConfigError {
//     fn from(err: String) -> JsonConfigError {
//         JsonConfigError::BundleNotFound(err)
//     }
// }

impl From<env::VarError> for JsonConfigError {
    fn from(err: env::VarError) -> JsonConfigError {
        JsonConfigError::EnvVar(err)
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
            JsonConfigError::ConfigDefinition(ref err) => write!(f, "Invalid configuration definition encountered. {}", err),
            //JsonConfigError::BundleNotFound(ref err) => write!(f, "The bundle {} does not exist.", err),
            JsonConfigError::EnvVar(ref err) => write!(f, "Encountered std::env::VarError: {}", err),
            JsonConfigError::Io(ref err) => write!(f, "Encountered std::io::Error: {}", err),
            JsonConfigError::SerdeJson(ref err) => write!(f, "Encountered serde_json::error::Error: {}", err),
        }
    }
}

impl error::Error for JsonConfigError {
    fn description(&self) -> &str {
        match *self {
            JsonConfigError::ConfigDefinition(ref err) => format!("Invalid configuration definition encountered. {}", err),
            //JsonConfigError::BundleNotFound(ref err) => format!("The bundle {} does not exist.", err),
            JsonConfigError::EnvVar(ref err) => error::Error::description(err),
            JsonConfigError::Io(ref err) => error::Error::description(err),
            JsonConfigError::SerdeJson(ref err) => error::Error::description(err),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            JsonConfigError::ConfigDefinition(ref err) => None,
            //JsonConfigError::BundleNotFound(ref err) => None,
            JsonConfigError::Io(ref err) => Some(err),
            JsonConfigError::SerdeJson(ref err) => Some(err),
            JsonConfigError::EnvVar(ref err) => Some(err),
        }
    }
}