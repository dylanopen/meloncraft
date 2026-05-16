use crate::ProtocolType;

impl ProtocolType for f32 {
    fn net_serialize(&self) -> Vec<u8> {
        return self.to_be_bytes().to_vec();
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        if data.len() < 4 {
            return Err(());
        }
        let arg_data = data.drain(0..4);
        let output = f32::from_be_bytes(arg_data.as_slice().try_into().map_err(|_| ())?);

        return Ok(output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn f32_serde() {
        let value = 3.1f32;
        let serialized = value.net_serialize();
        let deserialized = f32::net_deserialize(&mut serialized.clone()).unwrap();
        assert_eq!(value, deserialized);
    }
    #[test]
    fn f32_serde_negative() {
        let value = -2.5f32;
        let serialized = value.net_serialize();
        let deserialized = f32::net_deserialize(&mut serialized.clone()).unwrap();
        assert_eq!(value, deserialized);
    }
    #[test]
    fn f32_serde_zero() {
        let value = 0.0f32;
        let serialized = value.net_serialize();
        let deserialized = f32::net_deserialize(&mut serialized.clone()).unwrap();
        assert_eq!(value, deserialized);
    }
    #[test]
    fn f32_deserialize_insufficient_data() {
        let mut data = vec![0x00, 0x00, 0x00]; // 3 bytes instead of 4
        let deserialized = f32::net_deserialize(&mut data);
        assert!(deserialized.is_err());
    }
}

