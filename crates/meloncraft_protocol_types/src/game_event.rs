use meloncraft_core::{DemoEventType, GameMode, WeatherIntensity};

#[derive(Debug, Clone)]
pub enum GameEvent {
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

