pub mod deserialize;
pub mod nbt;
pub mod serialize;

mod protocol_type;
mod varint;
pub use protocol_type::ProtocolType;

pub use varint::VarInt;
