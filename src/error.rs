use std::{error, fmt, io};

/// One-based source position. `offset` is a zero-based byte offset.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[derive(Debug)]
pub enum Error {
    UnmatchedOpen {
        location: SourceLocation,
    },
    UnmatchedClose {
        location: SourceLocation,
    },
    PointerUnderflow {
        location: SourceLocation,
    },
    Io {
        location: Option<SourceLocation>,
        source: io::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnmatchedOpen { location } => write!(f, "unmatched '[' at {location}"),
            Self::UnmatchedClose { location } => write!(f, "unmatched ']' at {location}"),
            Self::PointerUnderflow { location } => {
                write!(f, "tape pointer moved left of cell 0 at {location}")
            }
            Self::Io {
                location: Some(location),
                source,
            } => write!(f, "I/O error at {location}: {source}"),
            Self::Io {
                location: None,
                source,
            } => write!(f, "I/O error: {source}"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}
