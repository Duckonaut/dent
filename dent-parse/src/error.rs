/// Error type returned by Dent.
///
/// This type is used for all errors returned by Dent, whether they are
/// parsing errors, IO errors or otherwise.
#[derive(Clone, PartialEq, Debug)]
pub enum Error {
    UnexpectedToken(String),
    UnknownFunction(String),
    UnexpectedEof,
    UnexpectedChar(char),
    Io(std::io::ErrorKind),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e.kind())
    }
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::UnexpectedToken(token) => format!("Unexpected token: {}", token),
            Error::UnknownFunction(name) => format!("Unknown function: {}", name),
            Error::UnexpectedEof => "Unexpected end of file".to_string(),
            Error::UnexpectedChar(c) => format!("Unexpected character: {}", c),
            Error::Io(e) => format!("IO error: {}", e),
        }
    }
}

/// Result type returned by Dent.
pub type Result<T> = std::result::Result<T, Error>;
