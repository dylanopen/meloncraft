use crate::{PrefixedArray, ProtocolBuffer, ProtocolType};
use meloncraft_player::{GameProfile, GameProfileProperties};

impl ProtocolType for GameProfile {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = Vec::new();
        output.extend(self.uuid.net_serialize());
        output.extend(self.username.net_serialize());
        let properties: Vec<GameProfileProperties> = Vec::new();
        output.extend(PrefixedArray(properties).net_serialize());
        output
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let uuid = data.net_deserialize()?;
        let username = data.net_deserialize()?;
        let _properties: PrefixedArray<GameProfileProperties> = data.net_deserialize()?;
        Ok(GameProfile { uuid, username })
    }
}

impl ProtocolType for GameProfileProperties {
    fn net_serialize(&self) -> Vec<u8> {
        todo!() // we don't need to do this yet I don't think
    }

    fn net_deserialize(_data: &mut Vec<u8>) -> Result<Self, ()> {
        todo!()
    }
}
