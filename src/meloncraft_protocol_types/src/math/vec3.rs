use bevy::math::Vec3;

use crate::{ProtocolBuffer as _, ProtocolType};

impl ProtocolType for Vec3 {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = Vec::new();
        output.extend(self.x.net_serialize());
        output.extend(self.y.net_serialize());
        output.extend(self.z.net_serialize());
        return output;
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let x = data.net_deserialize()?;
        let y = data.net_deserialize()?;
        let z = data.net_deserialize()?;
        return Ok(Vec3 { x, y, z });
    }
}
