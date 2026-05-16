use meloncraft_core::{DemoEventType, GameMode, WeatherIntensity};

use crate::ProtocolType;

#[derive(Debug, Clone)]
pub enum GameEventType {
    NoRespawnBlockAvailable,
    BeginRaining,
    EndRaining,
    ChangeGameMode(GameMode),
    WinGame(ShouldShowCredits),
    DemoEvent(DemoEventType),
    ArrowHitPlayer,
    RainLevelChange(WeatherIntensity),
    ThunderLevelChange(WeatherIntensity),
    PufferfishSting,
    ElderGuardianAppearance,
    EnableRespawnScreen(ShouldShowRespawnScreen),
    LimitedCrafting(bool),
    WaitForChunks,
}

#[derive(Debug, Clone)]
pub struct ShouldShowCredits(pub bool);

#[derive(Debug, Clone)]
pub struct ShouldShowRespawnScreen(pub bool);

impl ProtocolType for GameEventType {
    fn net_serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        let (id, value) = match self {
            GameEventType::NoRespawnBlockAvailable => (0, 0f32),
            GameEventType::BeginRaining => (1, 0f32),
            GameEventType::EndRaining => (2, 0f32),
            GameEventType::ChangeGameMode(mode) => (3, u8::from(*mode).into()),
            GameEventType::WinGame(ShouldShowCredits(show_credits)) => (4, if *show_credits { 1.0 } else { 0.0 }),
            GameEventType::DemoEvent(event) => (5, (*event).into()),
            GameEventType::ArrowHitPlayer => (6, 0f32),
            GameEventType::RainLevelChange(intensity) => (7, intensity.0),
            GameEventType::ThunderLevelChange(intensity) => (8, intensity.0),
            GameEventType::PufferfishSting => (9, 0f32),
            GameEventType::ElderGuardianAppearance => (10, 0f32),
            GameEventType::EnableRespawnScreen(ShouldShowRespawnScreen(show_screen)) => (11, if *show_screen { 0.0 } else { 1.0 }), // because... minecraft.
            GameEventType::LimitedCrafting(enabled) => (12, if *enabled { 1.0 } else { 0.0 }),
            GameEventType::WaitForChunks => (13, 0f32),
        };
        data.push(id);
        data.extend(value.net_serialize());
        data
    }

    fn net_deserialize(data: &mut Vec<u8>) -> Result<Self, ()> {
        // This code is untested and provided at the caller's risk.
        // Please report any bugs, panics or unexpected errors in a github issue!
        if data.is_empty() {
            return Err(());
        }
        let id = data.remove(0);
        let value = f32::net_deserialize(data)?;
        let event = match id {
            0 => GameEventType::NoRespawnBlockAvailable,
            1 => GameEventType::BeginRaining,
            2 => GameEventType::EndRaining,
            3 => GameEventType::ChangeGameMode(value.try_into()?),
            4 => GameEventType::WinGame(ShouldShowCredits(value != 0.0)),
            5 => GameEventType::DemoEvent(value.try_into()?),
            6 => GameEventType::ArrowHitPlayer,
            7 => GameEventType::RainLevelChange(WeatherIntensity(value)),
            8 => GameEventType::ThunderLevelChange(WeatherIntensity(value)),
            9 => GameEventType::PufferfishSting,
            10 => GameEventType::ElderGuardianAppearance,
            11 => GameEventType::EnableRespawnScreen(ShouldShowRespawnScreen(value == 0.0)),
            12 => GameEventType::LimitedCrafting(value != 0.0),
            13 => GameEventType::WaitForChunks,
            _ => return Err(()),
        };
        Ok(event)
    }
}

