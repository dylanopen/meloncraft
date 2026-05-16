use crate::{ProtocolBuffer as _, ProtocolType, VarInt};
use meloncraft_core::pause_menu::BuiltinPauseMenuLabel;

impl ProtocolType for BuiltinPauseMenuLabel {
    fn net_serialize(&self) -> Vec<u8> {
        VarInt(self.clone().into()).net_serialize()
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let num: VarInt = data.net_deserialize()?;
        num.0.try_into()
    }
}
