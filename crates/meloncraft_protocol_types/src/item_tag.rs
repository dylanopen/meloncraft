use crate::{PrefixedArray, ProtocolBuffer, ProtocolType};
use meloncraft_registry::ItemTag;

impl ProtocolType for ItemTag {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = self.name.net_serialize();
        output.extend(PrefixedArray(self.entries.clone()).net_serialize());
        output
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let name = data.net_deserialize()?;
        let entries = PrefixedArray::<i32>::net_deserialize(data)?.0;
        Ok(Self { name, entries })
    }
}
