use crate::ProtocolType;

impl ProtocolType for u16 {
    fn net_serialize(&self) -> Vec<u8> {
        self.to_be_bytes().to_vec()
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        if data.len() < 2 {
            return Err(());
        }

        let first_byte = data.remove(0);
        let second_byte = data.remove(0);

        let output: u16 = (first_byte as u16 * 256) + second_byte as u16;

        Ok(output)
    }
}
