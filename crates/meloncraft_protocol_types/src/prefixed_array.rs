use crate::{ProtocolBuffer, ProtocolType, VarInt};
use std::fmt::Debug;

pub struct PrefixedArray<T: ProtocolType> {
    pub values: Vec<T>,
}

impl<T: ProtocolType> ProtocolType for PrefixedArray<T> {
    fn net_serialize(&self) -> Vec<u8> {
        todo!()
    }
    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let length: VarInt = data.net_deserialize()?;
        let length = length.0;
        let target_length = data.len() - length as usize;
        let mut values = Vec::new();
        while data.len() > target_length {
            values.push(data.net_deserialize()?);
        }
        Ok(PrefixedArray { values })
    }
}

impl<T: ProtocolType> From<Vec<T>> for PrefixedArray<T> {
    fn from(values: Vec<T>) -> Self {
        Self { values }
    }
}
