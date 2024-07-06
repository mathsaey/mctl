use std::{error, fmt, io};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    SerialUnknown(String),
    SerialNoDevice(String),
    SerialInvalidInput(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "Serial device IO error: {}", err),
            Error::SerialUnknown(msg) => write!(f, "Unknown serial error: {}", msg),
            Error::SerialNoDevice(msg) => write!(f, "Serial device not available: {}", msg),
            Error::SerialInvalidInput(msg) => write!(f, "Invalid serial device parameter: {}", msg),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<serialport::Error> for Error {
    fn from(err: serialport::Error) -> Error {
        match err.kind {
            serialport::ErrorKind::Io(kind) => Error::Io(io::Error::new(kind, err.description)),
            serialport::ErrorKind::InvalidInput => Error::SerialInvalidInput(err.description),
            serialport::ErrorKind::NoDevice => Error::SerialNoDevice(err.description),
            serialport::ErrorKind::Unknown => Error::SerialUnknown(err.description),
        }
    }
}
