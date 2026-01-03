use crate::{ProtocolBuffer, ProtocolType, VarInt};

pub struct PrefixedArray<T: ProtocolType>(pub Vec<T>);

impl<T: ProtocolType> ProtocolType for PrefixedArray<T> {
    fn net_serialize(&self) -> Vec<u8> {
        let mut body = Vec::new();
        for value in &self.0 {
            body.append(&mut value.net_serialize());
        }
        let length = VarInt(body.len() as i32);
        let mut serial = length.net_serialize();
        serial.append(&mut body);
        serial
    }
    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let length: VarInt = data.net_deserialize()?;
        let length = length.0;
        let target_length = data.len() - length as usize;
        let mut values = Vec::new();
        while data.len() > target_length {
            values.push(data.net_deserialize()?);
        }
        Ok(PrefixedArray(values))
    }
}

impl<T: ProtocolType> From<Vec<T>> for PrefixedArray<T> {
    fn from(values: Vec<T>) -> Self {
        Self(values)
    }
}
