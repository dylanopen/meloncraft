use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Configuration,
    Play,
}

impl Display for ConnectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionState::Handshaking => write!(f, "Handshaking"),
            ConnectionState::Status => write!(f, "Status"),
            ConnectionState::Login => write!(f, "Login"),
            ConnectionState::Configuration => write!(f, "Configuration"),
            ConnectionState::Play => write!(f, "Play"),
        }
    }
}
