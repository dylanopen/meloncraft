use crate::{PrefixedArray, ProtocolBuffer as _, ProtocolType};
use meloncraft_player::{GameProfile, GameProfileProperties};

impl ProtocolType for GameProfile {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = Vec::new();
        output.extend(self.uuid.net_serialize());
        output.extend(self.username.net_serialize());
        let properties: Vec<GameProfileProperties> = Vec::new();
        output.extend(PrefixedArray(properties).net_serialize());
        return output;
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let uuid = data.net_deserialize()?;
        let username = data.net_deserialize()?;
        let _properties: PrefixedArray<GameProfileProperties> = data.net_deserialize()?;
        return Ok(GameProfile { uuid, username });
    }
}

#[expect(clippy::todo, reason = "GameProfileProperties serialization and deserialization is not implemented yet")]
impl ProtocolType for GameProfileProperties {
    fn net_serialize(&self) -> Vec<u8> {
        todo!("GameProfileProperties serialization");
    }

    fn net_deserialize(_data: &mut Vec<u8>) -> Result<Self, ()> {
        todo!("GameProfileProperties deserialization");
    }
}
