use crate::ProtocolType;
use meloncraft_core::pause_menu::PauseMenuLink;

impl ProtocolType for PauseMenuLink {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = self.label.net_serialize();
        output.extend(self.url.net_serialize());
        output
    }

    fn net_deserialize(_data: &mut Vec<u8>) -> Result<Self, ()> {
        todo!(
            "It seems unnecessary to implement deserialization,\
            as it will be complicated due to varying types, and\
            we will never receive this data, only send it."
        );
    }
}
