mod serialize;

use crate::ProtocolType;
use meloncraft_nbt::NbtTag;

impl ProtocolType for NbtTag {
    fn net_serialize(&self) -> Vec<u8> {
        serialize::root(self.clone())
    }

    fn net_deserialize(_data: &mut Vec<u8>) -> Result<Self, ()> {
        todo!()
    }
}
