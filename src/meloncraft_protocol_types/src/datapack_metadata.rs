use crate::{ProtocolBuffer as _, ProtocolType};
use meloncraft_core::datapack::DatapackMetadata;

impl ProtocolType for DatapackMetadata {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = self.namespace.net_serialize();
        output.extend(self.id.net_serialize());
        output.extend(self.version.net_serialize());
        return output;
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let namespace = data.net_deserialize()?;
        let id = data.net_deserialize()?;
        let version = data.net_deserialize()?;
        return Ok(Self {
            namespace,
            id,
            version,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn serde_datapack_metadata() {
        let metadata = DatapackMetadata {
            namespace: "example_namespace".to_string(),
            id: "example_id".to_string(),
            version: "42.9.8".to_string(),
        };
        let serialized = metadata.net_serialize();
        let deserialized = DatapackMetadata::net_deserialize(&mut serialized.clone()).unwrap();
        assert_eq!(metadata, deserialized);
    }
}
