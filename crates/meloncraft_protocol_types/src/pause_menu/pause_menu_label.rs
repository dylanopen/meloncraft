use crate::ProtocolType;
use meloncraft_core::pause_menu::PauseMenuLabel;

impl ProtocolType for PauseMenuLabel {
    fn net_serialize(&self) -> Vec<u8> {
        match self {
            PauseMenuLabel::Builtin(label_id) => label_id.net_serialize(),
            PauseMenuLabel::Custom(label_text) => label_text.net_serialize(),
        }
    }

    fn net_deserialize(_data: &mut Vec<u8>) -> Result<Self, ()> {
        todo!(
            "It seems unnecessary to implement deserialization, as it will be complicated due to varying types, and we will never receive this data, only send it."
        );
    }
}
