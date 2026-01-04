use crate::{ProtocolBuffer, ProtocolType};
use meloncraft_registry::RegistryEntry;

impl ProtocolType for RegistryEntry {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = self.id.net_serialize();
        output.extend(self.data.net_serialize());
        output
    }

    fn net_deserialize(bytes: &mut Vec<u8>) -> Result<Self, ()> {
        let id = bytes.net_deserialize()?;
        let data = bytes.net_deserialize()?;
        Ok(Self { id, data })
    }
}
