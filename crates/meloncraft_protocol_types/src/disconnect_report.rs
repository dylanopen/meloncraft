use crate::{ProtocolBuffer, ProtocolType};
use meloncraft_core::DisconnectReport;

impl ProtocolType for DisconnectReport {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = self.title.net_serialize();
        output.extend(self.description.net_serialize());
        output
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let title = data.net_deserialize()?;
        let description = data.net_deserialize()?;
        Ok(Self { title, description })
    }
}
