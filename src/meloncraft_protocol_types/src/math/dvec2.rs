use bevy::math::DVec2;

use crate::{ProtocolBuffer as _, ProtocolType};

impl ProtocolType for DVec2 {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = Vec::new();
        output.extend(self.x.net_serialize());
        output.extend(self.y.net_serialize());
        return output;
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let x = data.net_deserialize()?;
        let y = data.net_deserialize()?;
        return Ok(DVec2 { x, y });
    }
}
