use crate::{ProtocolBuffer as _, ProtocolType};
use meloncraft_registry::RegistryEntry;

impl ProtocolType for RegistryEntry {
    fn net_serialize(&self) -> Vec<u8> {
        let output = self.id.net_serialize();
        //output.extend(self.data.net_serialize());
        output
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let id = data.net_deserialize()?;
        let nbt_value = data.net_deserialize()?;
        Ok(Self { id, data: nbt_value })
    }
}
