use crate::ProtocolType;

impl ProtocolType for u32 {
    fn net_serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        if data.len() < 4 {
            return Err(());
        }
        let arg_data = data.drain(0..4);
        let output = u32::from_le_bytes(arg_data.as_slice().try_into().map_err(|_| ())?);

        Ok(output)
    }
}
