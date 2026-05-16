use crate::{ProtocolBuffer, ProtocolType, VarInt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixedArray<T: ProtocolType>(pub Vec<T>);

impl<T: ProtocolType> ProtocolType for PrefixedArray<T> {
    fn net_serialize(&self) -> Vec<u8> {
        let mut body = Vec::new();
        for value in &self.0 {
            body.append(&mut value.net_serialize());
        }
        let length = VarInt(self.0.len().try_into().unwrap());
        let mut serial = length.net_serialize();
        serial.append(&mut body);
        serial
    }
    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let length: VarInt = data.net_deserialize()?;
        let length = length.0;
        let mut values = Vec::with_capacity(length.try_into().unwrap());
        for _ in 0..length {
            let value = T::net_deserialize(data)?;
            values.push(value);
        }
        Ok(PrefixedArray(values))
    }
}

impl<T: ProtocolType> From<Vec<T>> for PrefixedArray<T> {
    fn from(values: Vec<T>) -> Self {
        Self(values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::byte::Byte;
    #[test]
    fn test_prefixed_array_serialize() {
        let array = PrefixedArray(vec![Byte(1), Byte(2), Byte(3)]);
        let serialized = array.net_serialize();
        let expected = vec![3, 1, 2, 3];
        assert_eq!(serialized, expected);
    }
    #[test]
    fn test_prefixed_array_deserialize() {
        let mut data = vec![3, 1, 2, 3];
        let deserialized = PrefixedArray::<Byte>::net_deserialize(&mut data).unwrap();
        let expected = PrefixedArray(vec![Byte(1), Byte(2), Byte(3)]);
        assert_eq!(deserialized.0, expected.0);
    }
    #[test]
    fn test_serialize_prefixed_array_string() {
        let array = PrefixedArray(vec!["hello".to_string(), "world".to_string()]);
        let serialized = array.net_serialize();
        let expected = vec![
            2, 5, b'h', b'e', b'l', b'l', b'o', 5, b'w', b'o', b'r', b'l', b'd',
        ];
        assert_eq!(serialized, expected);
    }
    #[test]
    fn test_deserialize_prefixed_array_string() {
        let mut data = vec![2, 5, b'h', b'e', b'l', b'l', b'o', 5, b'w', b'o', b'r', b'l', b'd'];
        let deserialized = PrefixedArray::<String>::net_deserialize(&mut data).unwrap();
        let expected = PrefixedArray(vec!["hello".to_string(), "world".to_string()]);
        assert_eq!(deserialized.0, expected.0);
    }
    #[test]
    fn test_prefixed_array_deserialize_empty() {
        let mut data = vec![0];
        let deserialized = PrefixedArray::<Byte>::net_deserialize(&mut data).unwrap();
        let expected = PrefixedArray(vec![]);
        assert_eq!(deserialized.0, expected.0);
    }
    #[test]
    fn test_single_item_prefixed_array_serialize() {
        let array = PrefixedArray(vec![Byte(42)]);
        let serialized = array.net_serialize();
        let expected = vec![1, 42];
        assert_eq!(serialized, expected);
    }
    #[test]
    fn test_single_item_prefixed_array_deserialize() {
        let mut data = vec![1, 42];
        let deserialized = PrefixedArray::<Byte>::net_deserialize(&mut data).unwrap();
        let expected = PrefixedArray(vec![Byte(42)]);
        assert_eq!(deserialized.0, expected.0);
    }
    #[test]
    fn test_empty_prefixed_array_serialize() {
        let array = PrefixedArray::<Byte>(vec![]);
        let serialized = array.net_serialize();
        let expected = vec![0];
        assert_eq!(serialized, expected);
    }
    #[test]
    fn test_empty_prefixed_array_deserialize() {
        let mut data = vec![0];
        let deserialized = PrefixedArray::<Byte>::net_deserialize(&mut data).unwrap();
        let expected = PrefixedArray(vec![]);
        assert_eq!(deserialized.0, expected.0);
    }
    #[test]
    fn test_prefixed_array_serde() {
        let array = PrefixedArray(vec![Byte(10), Byte(20), Byte(30)]);
        let mut serialized = array.net_serialize();
        let deserialized = PrefixedArray::<Byte>::net_deserialize(&mut serialized).unwrap();
        assert_eq!(array.0, deserialized.0);
    }
}

