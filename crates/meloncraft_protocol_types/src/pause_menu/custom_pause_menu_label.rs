use crate::{ProtocolBuffer, ProtocolType};
use meloncraft_core::pause_menu::CustomPauseMenuLabel;

impl ProtocolType for CustomPauseMenuLabel {
    fn net_serialize(&self) -> Vec<u8> {
        self.0.net_serialize()
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        Ok(Self(data.net_deserialize()?))
    }
}
