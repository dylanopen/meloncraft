use crate::ProtocolType;

impl ProtocolType for i64 {
    fn net_serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<i64, ()> {
        let output: i64 = i64::from_be_bytes(data[..8].try_into().map_err(|_| ())?);
        data.drain(0..7);
        Ok(output)
    }
}
