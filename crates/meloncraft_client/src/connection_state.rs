#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ConnectionState {
    Handshaking,
    Status,
    Login,
    Play,
}
