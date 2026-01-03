use crate::{ProtocolBuffer, ProtocolType};

impl<T: ProtocolType> ProtocolType for Option<T> {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = Vec::new();
        match self {
            None => output.extend(false.net_serialize()),
            Some(v) => {
                output.extend(true.net_serialize());
                output.extend(v.net_serialize());
            }
        }
        output
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let present = data.net_deserialize().unwrap();
        if present {
            Ok(Some(data.net_deserialize()?))
        } else {
            Ok(None)
        }
    }
}
