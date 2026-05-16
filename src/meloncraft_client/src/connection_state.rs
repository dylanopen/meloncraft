use core::fmt;
use core::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Configuration,
    Play,
}

impl Display for ConnectionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            ConnectionState::Handshaking => write!(f, "Handshaking"),
            ConnectionState::Status => write!(f, "Status"),
            ConnectionState::Login => write!(f, "Login"),
            ConnectionState::Configuration => write!(f, "Configuration"),
            ConnectionState::Play => write!(f, "Play"),
        };
    }
}
