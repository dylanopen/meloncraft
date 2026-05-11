// Inspiration for this enum was taken from Oxide, thank you <3
// https://git.thetxt.io/thetxt/oxide/src/lib/src/packets/clientbound/play.rs

use meloncraft_nbt::NbtTag;
use meloncraft_player::GameProfileProperties;

use crate::{PrefixedArray, ProtocolBuffer, ProtocolType};

#[derive(Debug, Clone)]
pub enum PlayerAction {
    	AddPlayer(AddPlayerAction),
	InitializeChat(Option<InitializeChatAction>),
	UpdateGameMode(i32),
	UpdateListed(bool),
	UpdateLatency(i32),
	UpdateDisplayName(Option<NbtTag>),
	UpdateListPriority(i32),
	UpdateHat(bool),
}

impl PlayerAction {
    pub fn mask(&self) -> u8 {
        match self {
            PlayerAction::AddPlayer(_) => 0x01,
            PlayerAction::InitializeChat(_) => 0x02,
            PlayerAction::UpdateGameMode(_) => 0x04,
            PlayerAction::UpdateListed(_) => 0x08,
            PlayerAction::UpdateLatency(_) => 0x10,
            PlayerAction::UpdateDisplayName(_) => 0x20,
            PlayerAction::UpdateListPriority(_) => 0x40,
            PlayerAction::UpdateHat(_) => 0x80,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AddPlayerAction {
    pub name: String,
    pub game_profile_properties: Vec<GameProfileProperties>,
}

#[derive(Debug, Clone)]
pub struct InitializeChatAction {
    pub session_id: u128,
    pub public_key_expiry_time: i64,
    pub encoded_public_key: Vec<u8>,
    pub public_key_signature: Vec<u8>,
}

impl ProtocolType for AddPlayerAction {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = Vec::new();
        output.extend(self.name.net_serialize());
        output.extend(PrefixedArray(self.game_profile_properties.clone()).net_serialize());
        output
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let name = data.net_deserialize()?;
        let game_profile_properties: PrefixedArray<GameProfileProperties> = data.net_deserialize()?;
        Ok(AddPlayerAction { name, game_profile_properties: game_profile_properties.0 })
    }
}

impl ProtocolType for InitializeChatAction {
    fn net_serialize(&self) -> Vec<u8> {
        let mut output = Vec::new();
        output.extend(self.session_id.net_serialize());
        output.extend(self.public_key_expiry_time.net_serialize());
        output.extend(PrefixedArray(self.encoded_public_key.clone()).net_serialize());
        output.extend(PrefixedArray(self.public_key_signature.clone()).net_serialize());
        output
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        let session_id = data.net_deserialize()?;
        let public_key_expiry_time = data.net_deserialize()?;
        let encoded_public_key: PrefixedArray<u8> = data.net_deserialize()?;
        let public_key_signature: PrefixedArray<u8> = data.net_deserialize()?;
        Ok(InitializeChatAction {
            session_id,
            public_key_expiry_time,
            encoded_public_key: encoded_public_key.0,
            public_key_signature: public_key_signature.0
        })
    }
}

/*
Action 	Mask 	Field Name 	Type 	Notes
Add Player 	0x01 	Name 	String (16) 	
Game profile properties 	Name 	Prefixed Array (16) 	String (64) 	
Value 	String (32767) 	
Signature 	Prefixed Optional String (1024) 	
Initialize Chat 	0x02 	Data 	Chat session ID 	Prefixed Optional 	UUID 	
Public key expiry time 	Long 	Key expiry time, as a UNIX timestamp in milliseconds. Only sent if Has Signature Data is true.
Encoded public key 	Prefixed Array (512) of Byte 	The player's public key, in bytes. Only sent if Has Signature Data is true.
Public key signature 	Prefixed Array (4096) of Byte 	The public key's digital signature. Only sent if Has Signature Data is true.
Update Game Mode 	0x04 	Game Mode 	VarInt
Update Listed 	0x08 	Listed 	Boolean 	Whether the player should be listed on the tab list.
Update Latency 	0x10 	Ping 	VarInt 	Measured in milliseconds.
Update Display Name 	0x20 	Display Name 	Prefixed Optional Text Component 	Only sent if Has Display Name is true.
Update List Priority 	0x40 	Priority 	VarInt 	See below.
Update Hat 	0x80 	Visible 	Boolean 	Whether the player's hat skin layer is shown. 
*/

impl ProtocolType for PlayerAction {
    fn net_serialize(&self) -> Vec<u8> {
        match self {
            PlayerAction::AddPlayer(action) => {
                action.net_serialize()
            },
            PlayerAction::InitializeChat(action) => {
                action.net_serialize()
            },
            PlayerAction::UpdateGameMode(game_mode) => {
                game_mode.net_serialize()
            },
            PlayerAction::UpdateListed(listed) => {
                listed.net_serialize()
            },
            PlayerAction::UpdateLatency(latency) => {
                latency.net_serialize()
            },
            PlayerAction::UpdateDisplayName(display_name) => {
                display_name.net_serialize()
            },
            PlayerAction::UpdateListPriority(priority) => {
                priority.net_serialize()
            },
            PlayerAction::UpdateHat(visible) => {
                visible.net_serialize()
            }
        }
    }

    fn net_deserialize(_data: &mut Vec<u8>) -> Result<Self, ()> {
        todo!() // we shouldn't need to do this, should be clientbound only
    }
}
