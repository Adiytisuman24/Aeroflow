#[derive(Debug)]
pub enum AeroError {
    ParseError(String),
    RuntimeError(String),
}

impl std::fmt::Display for AeroError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AeroError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            AeroError::RuntimeError(msg) => write!(f, "Runtime Error: {}", msg),
        }
    }
}

impl std::error::Error for AeroError {}
