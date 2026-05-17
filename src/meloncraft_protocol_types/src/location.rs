use bevy::math::IVec3;
use crate::ProtocolType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NetworkLocation(pub IVec3);

impl ProtocolType for NetworkLocation {
    fn net_serialize(&self) -> Vec<u8> {
        let long: i64 = ((i64::from(self.0.x) & 0x03FF_FFFF) << 38) | ((i64::from(self.0.z) & 0x03FF_FFFF) << 12) | (i64::from(self.0.y) & 0xFFF);
        return long.net_serialize();
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let long = i64::net_deserialize(data)?;
        return Ok(Self(IVec3::new(
            (long >> 38).try_into().unwrap(),
            (long << 52 >> 52).try_into().unwrap(),
            (long << 26 >> 38).try_into().unwrap(),
        )));
    }
}

#[cfg(test)]
mod tests {
    // these tests are based on https://minecraft.wiki/w/Java_Edition_protocol/Packets#Position
    #[test]
    fn location_serialization() {
        use super::*;
        let location = NetworkLocation(IVec3::new(18_357_644, 831, -20_882_616));
        let mut serialized = location.net_serialize();
        let serialized_long = u64::net_deserialize(&mut serialized).unwrap();
        let expected: u64 = 0b_0100_0110_0000_0111_0110_0011_0010_1100_0001_0101_1011_0100_1000_0011_0011_1111;
        assert_eq!(serialized_long, expected);
    }
    #[test]
    fn location_deserialization() {
        use super::*;
        let serialized_long: u64 = 0b_0100_0110_0000_0111_0110_0011_0010_1100_0001_0101_1011_0100_1000_0011_0011_1111;
        let mut serialized = serialized_long.net_serialize();
        let deserialized = NetworkLocation::net_deserialize(&mut serialized).unwrap();
        let expected = NetworkLocation(IVec3::new(18_357_644, 831, -20_882_616));
        assert_eq!(deserialized, expected);
    }
}

