/// Enum of different kinds of errors.
#[derive(Debug, Clone)]
pub enum ErrorKind {
    Parse,
    MissingValues,
    UnsupportedFormatSpec,
}

/// Error struct which implements `std::error::Error`.
#[derive(Debug)]
pub struct Error {
    message: String,
    kind: ErrorKind,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

impl Error {
    /// Create new parse error.
    pub fn new_parse(message: String) -> Self {
        Self {
            message: message,
            kind: ErrorKind::Parse,
        }
    }

    /// Create new missing values error.
    pub fn new_values(message: String) -> Self {
        Self {
            message: message,
            kind: ErrorKind::MissingValues,
        }
    }

    /// Create new unsupported format spec error.
    pub fn new_ufs(message: String) -> Self {
        Self {
            message: message,
            kind: ErrorKind::UnsupportedFormatSpec,
        }
    }

    /// Returns error message.
    pub fn message(&self) -> String {
        self.message.clone()
    }

    /// Returns `ErrorKind`
    pub fn kind(&self) -> ErrorKind {
        self.kind.clone()
    }
}
