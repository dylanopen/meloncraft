use crate::ProtocolType;

impl ProtocolType for bool {
    fn net_serialize(&self) -> Vec<u8> {
        match self {
            false => vec![0],
            true => vec![1],
        }
    }
    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        data.reverse();
        let value = data.pop().ok_or(())?;
        data.reverse();

        match value {
            0x00 => Ok(false),
            0x01 => Ok(true),
            _ => Err(()),
        }
    }
}
