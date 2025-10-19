/// The expiration kind. See https://valkey.io/commands/expire/ for more information.
#[derive(Debug, Clone)]
pub enum ExpiryKind {
    Nx,
    Xx,
    Gt,
    Lt,
}

impl std::fmt::Display for ExpiryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Nx => "NX",
            Self::Xx => "XX",
            Self::Gt => "GT",
            Self::Lt => "LT",
        };
        write!(f, "{value}")
    }
}
