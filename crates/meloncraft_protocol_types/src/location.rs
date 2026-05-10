use crate::ProtocolType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NetworkLocation {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl ProtocolType for NetworkLocation {
    fn net_serialize(&self) -> Vec<u8> {
        let long: i64 = ((self.x as i64 & 0x3FFFFFF) << 38) | ((self.z as i64 & 0x3FFFFFF) << 12) | (self.y as i64 & 0xFFF);
        long.net_serialize()
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let long = i64::net_deserialize(data)?;
        Ok(Self {
            x: (long >> 38) as i32,
            y: (long << 52 >> 52) as i32,
            z: (long << 26 >> 38) as i32,
        })
    }
}

mod tests {
    // these tests are based on https://minecraft.wiki/w/Java_Edition_protocol/Packets#Position
    #[test]
    fn test_location_serialization() {
        use super::*;
        let location = NetworkLocation { x: 18357644, y: 831, z: -20882616 };
        let serialized = location.net_serialize();
        let serialized_long = u64::net_deserialize(&mut serialized.clone()).unwrap();
        let expected: u64 = 0b0100011000000111011000110010110000010101101101001000001100111111;
        assert_eq!(serialized_long, expected);
    }
    #[test]
    fn test_location_deserialization() {
        use super::*;
        let serialized_long: u64 = 0b0100011000000111011000110010110000010101101101001000001100111111;
        let serialized = serialized_long.net_serialize();
        let deserialized = NetworkLocation::net_deserialize(&mut serialized.clone()).unwrap();
        let expected = NetworkLocation { x: 18357644, y: 831, z: -20882616 };
        assert_eq!(deserialized, expected);
    }
}

