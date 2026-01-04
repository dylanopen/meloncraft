use crate::{PrefixedArray, ProtocolBuffer, ProtocolType};
use meloncraft_registry::{ItemTag, RegistryTags};

impl ProtocolType for RegistryTags {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = self.registry_name.net_serialize();
        output.extend(PrefixedArray(self.tags).net_serialize());
        output
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let registry_name = data.net_deserialize()?;
        let tags = PrefixedArray::<ItemTag>::net_deserialize(data)?.0;
        Ok(Self {
            registry_name,
            tags,
        })
    }
}
