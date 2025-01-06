/// Our generic error type
#[derive(Debug)]
pub enum Error {
    CouldNotDetermineOS,
    DateParse,
    EnvVar,
    IO,
    JSON,
    NonZeroExitStatus,
    Utf8String,
    UnsupportedOS,
}

impl From<std::io::Error> for Error {
    fn from(_e: std::io::Error) -> Self {
        Error::IO
    }
}

impl From<time::error::Parse> for Error {
    fn from(_e: time::error::Parse) -> Self {
        Error::DateParse
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(_e: std::string::FromUtf8Error) -> Self {
        Error::Utf8String
    }
}

impl From<std::env::VarError> for Error {
    fn from(_e: std::env::VarError) -> Self {
        Error::EnvVar
    }
}

impl From<serde_json::Error> for Error {
    fn from(_e: serde_json::Error) -> Self {
        Error::JSON
    }
}
