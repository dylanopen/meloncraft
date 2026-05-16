use crate::{ProtocolType, VarInt};

impl ProtocolType for String {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = VarInt(self.len().try_into().unwrap()).net_serialize();
        output.append(&mut self.as_bytes().to_vec());
        output
    }
    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let length: usize = VarInt::net_deserialize(data)?.0.try_into().unwrap();
        if data.len() < length {
            return Err(());
        }
        let string_bytes = &data.clone()[..length];
        data.drain(..length);

        String::from_utf8(string_bytes.to_vec()).map_err(|_| ())
    }
}
