use crate::{ProtocolBuffer, ProtocolType};
use meloncraft_core::datapack::DatapackMetadata;

impl ProtocolType for DatapackMetadata {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = self.namespace.net_serialize();
        output.extend(self.id.net_serialize());
        output.extend(self.version.net_serialize());
        output
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let namespace = data.net_deserialize()?;
        let id = data.net_deserialize()?;
        let version = data.net_deserialize()?;
        Ok(Self {
            namespace,
            id,
            version,
        })
    }
}
