use crate::types::error_kind::ErrorKind;

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl Error {
    pub fn new(kind: ErrorKind, msg: impl Into<String>) -> Self {
        Self {
            kind,
            message: msg.into(),
            source: None,
        }
    }

    pub fn with_source(
        kind: ErrorKind,
        msg: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            kind,
            message: msg.into(),
            source: Some(Box::new(source)),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.source.is_some() {
            write!(
                f,
                "{:?} {} {:?}",
                self.kind,
                self.message,
                self.source.as_ref()
            )
        } else {
            write!(f, "{:?} {}", self.kind, self.message)
        }
    }
}
