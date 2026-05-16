use crate::{PrefixedArray, ProtocolBuffer as _, ProtocolType};
use meloncraft_registry::{ItemTag, RegistryTags};

impl ProtocolType for RegistryTags {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = self.registry_name.net_serialize();
        output.extend(PrefixedArray(self.tags.clone()).net_serialize());
        return output;
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let registry_name = data.net_deserialize()?;
        let tags = PrefixedArray::<ItemTag>::net_deserialize(data)?.0;
        return Ok(Self {
            registry_name,
            tags,
        });
    }
}
