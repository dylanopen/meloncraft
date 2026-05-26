// Inspiration for this serialization/deserialization was taken from Oxide, thank you <3
// https://git.thetxt.io/thetxt/oxide/src/lib/src/packets/clientbound/play.rs

use meloncraft_player::GameProfileProperties;
use meloncraft_player::client_action::{AddPlayerAction, ClientPlayerAction, InitializeChatAction};

use crate::{PrefixedArray, ProtocolBuffer as _, ProtocolType, VarInt};

impl ProtocolType for AddPlayerAction {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = Vec::new();
        output.extend(self.name.net_serialize());
        output.extend(PrefixedArray(self.game_profile_properties.clone()).net_serialize());
        return output;
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let name = data.net_deserialize()?;
        let game_profile_properties: PrefixedArray<GameProfileProperties> =
            data.net_deserialize()?;
        return Ok(AddPlayerAction {
            name,
            game_profile_properties: game_profile_properties.0,
        });
    }
}

impl ProtocolType for InitializeChatAction {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = Vec::new();
        output.extend(self.session_id.net_serialize());
        output.extend(self.public_key_expiry_time.net_serialize());
        output.extend(PrefixedArray(self.encoded_public_key.clone()).net_serialize());
        output.extend(PrefixedArray(self.public_key_signature.clone()).net_serialize());
        return output;
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let session_id = data.net_deserialize()?;
        let public_key_expiry_time = data.net_deserialize()?;
        let encoded_public_key: PrefixedArray<u8> = data.net_deserialize()?;
        let public_key_signature: PrefixedArray<u8> = data.net_deserialize()?;
        return Ok(InitializeChatAction {
            session_id,
            public_key_expiry_time,
            encoded_public_key: encoded_public_key.0,
            public_key_signature: public_key_signature.0,
        });
    }
}

impl ProtocolType for ClientPlayerAction {
    fn net_serialize(&self) -> Vec<u8> {
        return match self {
            ClientPlayerAction::AddPlayer(action) => action.net_serialize(),
            ClientPlayerAction::InitializeChat(action) => action.net_serialize(),
            ClientPlayerAction::UpdateGameMode(game_mode) => VarInt(*game_mode).net_serialize(),
            ClientPlayerAction::UpdateListed(listed) => listed.net_serialize(),
            ClientPlayerAction::UpdateLatency(latency) => VarInt(*latency).net_serialize(),
            ClientPlayerAction::UpdateDisplayName(display_name) => display_name.net_serialize(),
            ClientPlayerAction::UpdateListPriority(priority) => VarInt(*priority).net_serialize(),
            ClientPlayerAction::UpdateHat(visible) => visible.net_serialize(),
        };
    }

    #[expect(
        clippy::panic,
        clippy::panic_in_result_fn,
        reason = "This feature is currently unimplemented. It should panic when called, but it should not be used in the first place."
    )]
    fn net_deserialize(_data: &mut Vec<u8>) -> Result<Self, ()> {
        panic!(
            "ClientPlayerAction cannot currently be deserialized. Please implement this if you need it, or open an issue requesting it."
        );
    }
}
