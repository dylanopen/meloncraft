use crate::ProtocolType;
use meloncraft_core::pause_menu::PauseMenuLabel;

impl ProtocolType for PauseMenuLabel {
    fn net_serialize(&self) -> Vec<u8> {
        return match self {
            PauseMenuLabel::Builtin(label_id) => label_id.net_serialize(),
            PauseMenuLabel::Custom(label_text) => label_text.net_serialize(),
        };
    }

    #[expect(
        clippy::panic,
        clippy::panic_in_result_fn,
        reason = "This struct shouldn't ever need to be deserialized."
    )]
    fn net_deserialize(_data: &mut Vec<u8>) -> Result<Self, ()> {
        panic!(
            "Tried to deserialize a PauseMenuLabel, but this struct can't currently be deserialized. If you need this struct to be deserialized, please open an issue or, better yet, a pull request that implements deserialization for this struct."
        );
    }
}
