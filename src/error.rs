extern crate serde_json;

use std::error;
use std::fmt;
use std::io;
use std::env;

#[derive(Debug)]
pub enum ErrorKind{
    ConfigDefinition,
    //BundleNotFound(String),
    EnvVar, //(env::VarError),
    Io, //(io::Error),
    SerdeJson //(serde_json::error::Error)    
}

#[derive(Debug)]
pub struct ConfigDefinitionError {
    err: String
}

impl fmt::Display for ConfigDefinitionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid configuration definition. {}", self.err)
    }
}

impl error::Error for ConfigDefinitionError {
    fn description(&self) -> &str {
        let desc = format!("Invalid configuration definition. {}", &self.err[..]);
        &desc
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}


#[derive(Debug)]
pub struct JsonConfigError<'a> {
    kind: ErrorKind,
    err: &'a error::Error
}

impl<'a> JsonConfigError<'a> {
    pub fn new(kind: ErrorKind, err: &error::Error) -> JsonConfigError {
        JsonConfigError {
            kind: kind,
            err: err
        }
    }
}

impl<'a> From<ConfigDefinitionError> for JsonConfigError<'a> {
    fn from(err: ConfigDefinitionError) -> JsonConfigError<'a> {
        JsonConfigError::new(ErrorKind::ConfigDefinition, &err)
    }
}

// impl From<String> for JsonConfigError {
//     fn from(err: String) -> JsonConfigError {
//         JsonConfigError::BundleNotFound(err)
//     }
// }

impl<'a> From<env::VarError> for JsonConfigError<'a> {
    fn from(err: env::VarError) -> JsonConfigError<'a> {
        JsonConfigError::new(ErrorKind::EnvVar, &err)
    }
}

impl<'a> From<io::Error> for JsonConfigError<'a> {
    fn from(err: io::Error) -> JsonConfigError<'a> {
        JsonConfigError::new(ErrorKind::Io, &err)
    }
}

impl<'a> From<serde_json::error::Error> for JsonConfigError<'a> {
    fn from(err: serde_json::error::Error) -> JsonConfigError<'a> {
        JsonConfigError::new(ErrorKind::SerdeJson, &err)
    }
}

impl<'a> fmt::Display for JsonConfigError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            ErrorKind::ConfigDefinition => write!(f, "Invalid configuration definition encountered. {}", self.err),
            //JsonConfigError::BundleNotFound(ref err) => write!(f, "The bundle {} does not exist.", err),
            ErrorKind::EnvVar => write!(f, "Encountered std::env::VarError: {}", self.err),
            ErrorKind::Io => write!(f, "Encountered std::io::Error: {}", self.err),
            ErrorKind::SerdeJson => write!(f, "Encountered serde_json::error::Error: {}", self.err),
        }
    }
}

impl<'a> error::Error for JsonConfigError<'a> {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::ConfigDefinition => error::Error::description(self.err),
            //JsonConfigError::BundleNotFound(ref err) => format!("The bundle {} does not exist.", err),
            ErrorKind::EnvVar => error::Error::description(self.err),
            ErrorKind::Io => error::Error::description(self.err),
            ErrorKind::SerdeJson => error::Error::description(self.err),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self.kind {
            ErrorKind::ConfigDefinition => Some(self.err),
            //JsonConfigError::BundleNotFound(ref err) => None,
            ErrorKind::Io => Some(self.err),
            ErrorKind::SerdeJson => Some(self.err),
            ErrorKind::EnvVar => Some(self.err),
        }
    }
}