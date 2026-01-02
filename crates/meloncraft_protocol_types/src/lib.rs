pub mod nbt;

mod boolean;
mod i64;
mod protocol_type;
mod string;
mod u16;
mod uuid;
mod varint;

pub use protocol_type::ProtocolType;
pub use varint::VarInt;
