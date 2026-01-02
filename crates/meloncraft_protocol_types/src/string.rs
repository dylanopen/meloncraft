use crate::{ProtocolType, VarInt};

impl ProtocolType for String {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = VarInt(self.len() as i32).net_serialize();
        output.append(&mut self.as_bytes().to_vec());
        output
    }
    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let length = VarInt::net_deserialize(data)?.0;
        if data.len() < length as usize {
            return Err(());
        }
        let string_bytes = &data.clone()[..length as usize];
        data.drain(..length as usize);

        String::from_utf8(string_bytes.to_vec()).map_err(|_| ())
    }
}
