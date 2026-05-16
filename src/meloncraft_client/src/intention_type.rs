use core::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum IntentionType {
    Status,
    Login,
    Transfer,
}

impl fmt::Display for IntentionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            IntentionType::Status => write!(f, "Status"),
            IntentionType::Login => write!(f, "Login"),
            IntentionType::Transfer => write!(f, "Transfer"),
        };
    }
}
