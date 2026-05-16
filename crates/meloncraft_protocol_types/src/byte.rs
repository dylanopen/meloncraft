use crate::{ProtocolBuffer as _, ProtocolType};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Byte(pub i8);

impl ProtocolType for Byte {
    fn net_serialize(&self) -> Vec<u8> {
        return self.0.net_serialize();
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        return Ok(Self(data.net_deserialize()?));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn byte_serialize_neg128() {
        let value = Byte(-128);
        let serialized = value.net_serialize();
        assert_eq!(serialized, vec![128]);
    }
    #[test]
    fn byte_serialize_127() {
        let value = Byte(127);
        let serialized = value.net_serialize();
        assert_eq!(serialized, vec![127]);
    }
    #[test]
    fn byte_serialize_0() {
        let value = Byte(0);
        let serialized = value.net_serialize();
        assert_eq!(serialized, vec![0]);
    }
    #[test]
    fn byte_deserialize_neg128() {
        let mut data = vec![128];
        let deserialized = Byte::net_deserialize(&mut data).unwrap();
        assert_eq!(deserialized, Byte(-128));
    }
    #[test]
    fn byte_deserialize_127() {
        let mut data = vec![127];
        let deserialized = Byte::net_deserialize(&mut data).unwrap();
        assert_eq!(deserialized, Byte(127));
    }
    #[test]
    fn byte_deserialize_0() {
        let mut data = vec![0];
        let deserialized = Byte::net_deserialize(&mut data).unwrap();
        assert_eq!(deserialized, Byte(0));
    }
    #[test]
    fn byte_deserialize_invalid() {
        let mut data = vec![255];
        let deserialized = Byte::net_deserialize(&mut data).unwrap();
        assert_eq!(deserialized, Byte(-1));
    }
    #[test]
    fn byte_deserialize_empty() {
        let mut data = vec![];
        let deserialized = Byte::net_deserialize(&mut data);
        assert!(deserialized.is_err());
    }
}
