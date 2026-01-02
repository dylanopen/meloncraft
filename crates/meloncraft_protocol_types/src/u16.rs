use crate::ProtocolType;

impl ProtocolType for u16 {
    fn net_serialize(&self) -> Vec<u8> {
        vec![(self / 256) as u8, (self % 256) as u8]
    }

    fn net_deserialize(bytes: &mut Vec<u8>) -> Result<Self, ()> {
        let byte_1 = bytes.pop().ok_or(())?;
        let byte_2 = bytes.pop().ok_or(())?;
        Ok(byte_1 as u16 * 256u16 + byte_2 as u16)
    }
}
