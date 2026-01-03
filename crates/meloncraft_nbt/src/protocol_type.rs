use crate::serialize;
use crate::tag::NbtTag;
use meloncraft_protocol_types::ProtocolType;

impl ProtocolType for NbtTag {
    fn net_serialize(&self) -> Vec<u8> {
        serialize::root(self.clone())
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        todo!()
    }
}
