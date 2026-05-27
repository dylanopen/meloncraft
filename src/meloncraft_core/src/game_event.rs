use crate::{DemoEventType, GameMode, WeatherIntensity};

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShouldShowCredits(pub bool);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShouldShowRespawnScreen(pub bool);
