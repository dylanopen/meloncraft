use crate::clientbound_packet::ClientboundPacket;
use bevy::ecs::message::Message;
use bevy::prelude::Entity;
use meloncraft_bossbar::color::BossbarColor;
use meloncraft_bossbar::division::BossbarDivision;
use meloncraft_bossbar::flags::{BossbarCreatesFog, BossbarDarkensSky, BossbarIsDragon};
use meloncraft_bossbar::title::BossbarTitle;
use meloncraft_client::connection_state::ConnectionState;
use meloncraft_entity::Uuid;
use meloncraft_entity::health::current::CurrentHealth;
use meloncraft_protocol_types::{ProtocolType as _, VarInt};

#[derive(Message, Debug, Clone)]
pub struct ClientboundBossEvent {
    pub client: Entity,
    pub uuid: Uuid,
    pub action: BossEventAction,
}

impl ClientboundPacket for ClientboundBossEvent {
    fn id() -> i32 {
        return 0x09;
    }

    fn state() -> ConnectionState {
        return ConnectionState::Play;
    }

    fn client(&self) -> Entity {
        return self.client;
    }

    fn data(&self, data: &mut Vec<u8>) {
        data.extend(self.uuid.net_serialize());
        match self.action.clone() {
            BossEventAction::Add {
                title,
                health,
                color,
                division,
                darkens_sky,
                is_dragon,
                creates_fog,
            } => {
                data.extend(VarInt(0).net_serialize());
                data.extend(title.0.net_serialize());
                data.extend(health.0.net_serialize());
                data.extend(VarInt(color.into()).net_serialize());
                data.extend(VarInt(division.into()).net_serialize());

                let mut bit_flags = 0_u8;
                if darkens_sky.0 {
                    bit_flags |= 0x_1;
                }
                if is_dragon.0 {
                    bit_flags |= 0x_2;
                }
                if creates_fog.0 {
                    bit_flags |= 0x_4;
                }
                data.push(bit_flags);
            }
            BossEventAction::Remove => data.extend(VarInt(1).net_serialize()),
            BossEventAction::UpdateHealth { new_health } => {
                data.extend(VarInt(2).net_serialize());
                data.extend(new_health.0.net_serialize());
            }
            BossEventAction::UpdateTitle { new_title } => {
                data.extend(VarInt(3).net_serialize());
                data.extend(new_title.0.net_serialize());
            }
            BossEventAction::UpdateStyle {
                new_color,
                new_division,
            } => {
                data.extend(VarInt(4).net_serialize());
                data.extend(VarInt(new_color.into()).net_serialize());
                data.extend(VarInt(new_division.into()).net_serialize());
            }
            BossEventAction::UpdateFlags {
                new_darkens_sky,
                new_is_dragon,
                new_creates_fog,
            } => {
                data.extend(VarInt(5).net_serialize());
                let mut bit_flags = 0_u8;
                if new_darkens_sky.0 {
                    bit_flags |= 0x_1;
                }
                if new_is_dragon.0 {
                    bit_flags |= 0x_2;
                }
                if new_creates_fog.0 {
                    bit_flags |= 0x_4;
                }
                data.push(bit_flags);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum BossEventAction {
    Add {
        title: BossbarTitle,
        health: CurrentHealth,
        color: BossbarColor,
        division: BossbarDivision,
        darkens_sky: BossbarDarkensSky,
        is_dragon: BossbarIsDragon,
        creates_fog: BossbarCreatesFog,
    },
    Remove,
    UpdateHealth {
        new_health: CurrentHealth,
    },
    UpdateTitle {
        new_title: BossbarTitle,
    },
    UpdateStyle {
        new_color: BossbarColor,
        new_division: BossbarDivision,
    },
    UpdateFlags {
        new_darkens_sky: BossbarDarkensSky,
        new_is_dragon: BossbarIsDragon,
        new_creates_fog: BossbarCreatesFog,
    },
}
