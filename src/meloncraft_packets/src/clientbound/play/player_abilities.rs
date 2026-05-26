use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_player::{CanFly, CanInstantBreak, FlySpeed, FovModifier, Invulnerable, IsFlying};
use crate::network_messages::ClientboundNetworkPacket;
use meloncraft_protocol_types::ProtocolType as _;
use crate::clientbound_packet::ClientboundPacket;

/// Send the client their 'abilities', see the fields for what this includes.
#[derive(Message, Debug, Clone)]
pub struct ClientboundPlayerAbilities {
    pub client: Entity,

    /// True if the player cannot (usually) take damage, false otherwise.
    pub invulnerable: Invulnerable,

    /// True if the player is *currently* flying, false if they are not currently flying.
    pub is_flying: IsFlying,

    /// True if the player is *able* to fly (e.g. they have the perms), false if they cannot.
    pub allow_flying: CanFly,

    /// True if the player can insta-break any block, like they are in creative mode, false
    /// otherwise.
    pub instant_break: CanInstantBreak,

    /// The player's flying speed as a float, see [`FlySpeed`].
    pub fly_speed: FlySpeed,

    /// The player's FOV modifier as a float, see [`FovModifier`].
    pub fov_modifier: FovModifier,
}

impl ClientboundPacket for ClientboundPlayerAbilities {
    fn id() -> i32 {
        return 0x3E
    }

    fn state() -> ConnectionState {
        return ConnectionState::Play
    }


    fn client(&self) -> Entity {
        return self.client;
    }

    fn serialize(&self) -> Option<ClientboundNetworkPacket> {
        let mut data = Vec::new();

        data.extend((
                u8::from(self.invulnerable.0)
                | (u8::from(self.is_flying.0) << 1)
                | (u8::from(self.allow_flying.0) << 2)
                | (u8::from(self.instant_break.0) << 3)
        ).net_serialize());
        data.extend(self.fly_speed.0.net_serialize());
        data.extend(self.fov_modifier.0.net_serialize());

        return Some(ClientboundNetworkPacket {
            client: self.client,
            id: Self::id(),
            data,
        })
    }
}

