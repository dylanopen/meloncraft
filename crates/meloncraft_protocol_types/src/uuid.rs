use crate::ProtocolType;
use meloncraft_player::Uuid;

impl ProtocolType for Uuid {
    fn net_serialize(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        if data.len() < 16 {
            return Err(());
        }
        let drained_data = data.drain(0..16);
        let output: u128 = u128::from_be_bytes(drained_data.as_slice().try_into().map_err(|_| ())?);
        Ok(Uuid(output))
    }
}
