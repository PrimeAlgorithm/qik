use std::{error::Error, fmt};

/// Stable runtime error categories used as process exit codes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum ErrorKind {
    Transport = 3,
    Timeout = 4,
    Tls = 5,
    HttpStatus = 6,
    Output = 7,
    ResponseTooLarge = 8,
}

#[derive(Debug)]
pub struct QikError {
    kind: ErrorKind,
    source: anyhow::Error,
}

impl QikError {
    pub fn new(kind: ErrorKind, source: impl Into<anyhow::Error>) -> Self {
        Self {
            kind,
            source: source.into(),
        }
    }

    pub fn exit_code(&self) -> i32 {
        self.kind as i32
    }

    pub fn from_reqwest(error: reqwest::Error, context: &str) -> Self {
        let kind = if error.is_timeout() {
            ErrorKind::Timeout
        } else if error_chain_mentions_tls(&error) {
            ErrorKind::Tls
        } else {
            ErrorKind::Transport
        };
        Self::new(kind, anyhow::anyhow!("{context}: {error}"))
    }
}

fn error_chain_mentions_tls(error: &reqwest::Error) -> bool {
    let mut current: Option<&(dyn Error + 'static)> = Some(error);
    while let Some(cause) = current {
        let message = cause.to_string().to_ascii_lowercase();
        if ["tls", "ssl", "certificate", "handshake"]
            .iter()
            .any(|term| message.contains(term))
        {
            return true;
        }
        current = cause.source();
    }
    false
}

impl fmt::Display for QikError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{:#}", self.source)
    }
}

impl Error for QikError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.source.as_ref())
    }
}
