use crate::{ProtocolBuffer as _, ProtocolType};

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
        return output;
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let present = data.net_deserialize().unwrap();
        if present {
            return Ok(Some(data.net_deserialize()?));
        }
        return Ok(None); // this looks really stupid, but it's saying 'successfully deserialized, the value was null'
    }
}
