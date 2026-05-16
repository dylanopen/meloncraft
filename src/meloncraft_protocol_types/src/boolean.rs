use crate::ProtocolType;

impl ProtocolType for bool {
    fn net_serialize(&self) -> Vec<u8> {
        if *self {
            return vec![1];
        }
        return vec![0];
    }
    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        data.reverse();
        let value = data.pop().ok_or(())?;
        data.reverse();

        match value {
            0x00 => return Ok(false),
            0x01 => return Ok(true),
            _ => return Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn boolean_serialize_true() {
        let value = true;
        let serialized = value.net_serialize();
        assert_eq!(serialized, vec![1]);
    }
    #[test]
    fn boolean_serialize_false() {
        let value = false;
        let serialized = value.net_serialize();
        assert_eq!(serialized, vec![0]);
    }
    #[test]
    fn boolean_deserialize_true() {
        let mut data = vec![1];
        let deserialized = bool::net_deserialize(&mut data).unwrap();
        assert!(deserialized);
    }
    #[test]
    fn boolean_deserialize_false() {
        let mut data = vec![0];
        let deserialized = bool::net_deserialize(&mut data).unwrap();
        assert!(!deserialized);
    }
    #[test]
    fn boolean_deserialize_invalid() {
        let mut data = vec![2];
        let deserialized = bool::net_deserialize(&mut data);
        assert!(deserialized.is_err());
    }
}
