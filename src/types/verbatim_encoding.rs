use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerbatimEncoding {
    Txt,             // Plain UTF-8 text
    Mkd,             // Markdown
    Htm,             // HTML
    Xml,             // XML data
    Csv,             // CSV-formatted
    Bin,             // Binary data
    Log,             // Log entry
    Raw,             // Raw or unknown text
    Unknown(String), // Custom or unrecognized prefix
}
impl VerbatimEncoding {
    pub fn from(prefix: &str) -> Self {
        match prefix.to_ascii_lowercase().as_str() {
            "txt" => Self::Txt,
            "mkd" => Self::Mkd,
            "htm" => Self::Htm,
            "xml" => Self::Xml,
            "csv" => Self::Csv,
            "bin" => Self::Bin,
            "log" => Self::Log,
            "raw" => Self::Raw,
            other => Self::Unknown(other.to_string()),
        }
    }
}

impl std::fmt::Display for VerbatimEncoding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Txt => write!(f, "txt"),
            Self::Mkd => write!(f, "mkd"),
            Self::Htm => write!(f, "htm"),
            Self::Xml => write!(f, "xml"),
            Self::Csv => write!(f, "csv"),
            Self::Bin => write!(f, "bin"),
            Self::Log => write!(f, "log"),
            Self::Raw => write!(f, "raw"),
            Self::Unknown(s) => write!(f, "{}", s),
        }
    }
}
