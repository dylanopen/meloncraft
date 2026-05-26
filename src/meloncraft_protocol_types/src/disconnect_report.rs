use crate::{ProtocolBuffer as _, ProtocolType};
use meloncraft_core::DisconnectReport;

impl ProtocolType for DisconnectReport {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = self.title.net_serialize();
        output.extend(self.description.net_serialize());
        return output;
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let title = data.net_deserialize()?;
        let description = data.net_deserialize()?;
        return Ok(Self { title, description });
    }
}
