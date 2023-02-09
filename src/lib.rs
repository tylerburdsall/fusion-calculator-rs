#[derive(Debug)]
pub enum FusionCalculatorError {
    /// A persona was not found under that name
    PersonaNotFound(String),

    /// The attempted fusion is not possible
    InvalidFusion(String),
}

impl std::fmt::Display for FusionCalculatorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FusionCalculatorError::PersonaNotFound(e) | FusionCalculatorError::InvalidFusion(e) => {
                write!(f, "{e}")
            }
        }
    }
}

impl std::error::Error for FusionCalculatorError {}
