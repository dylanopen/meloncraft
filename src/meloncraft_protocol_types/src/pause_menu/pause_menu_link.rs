use crate::ProtocolType;
use meloncraft_core::pause_menu::PauseMenuLink;

impl ProtocolType for PauseMenuLink {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = self.label.net_serialize();
        output.extend(self.url.net_serialize());
        return output;
    }

    #[expect(
        clippy::panic,
        clippy::panic_in_result_fn,
        reason = "This struct shouldn't ever need to be deserialized."
    )]
    fn net_deserialize(_data: &mut Vec<u8>) -> Result<Self, ()> {
        panic!(
            "Tried to deserialize a PauseMenuLink, but this struct can't currently be deserialized. If you need this struct to be deserialized, please open an issue or, better yet, a pull request that implements deserialization for this struct."
        );
    }
}
