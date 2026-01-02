use crate::ProtocolType;

impl ProtocolType for i16 {
    fn net_serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        if data.len() < 2 {
            return Err(());
        }
        let arg_data = data.drain(0..2);
        let output = i16::from_le_bytes(arg_data.as_slice().try_into().map_err(|_| ())?);

        Ok(output)
    }
}
