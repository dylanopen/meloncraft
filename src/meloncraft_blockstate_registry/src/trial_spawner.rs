use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrialSpawner {
    pub r#trial_spawner_state: TrialSpawnerState,
    pub ominous: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrialSpawnerState {
    Inactive,
    WaitingForPlayers,
    Active,
    WaitingForRewardEjection,
    EjectingReward,
    Cooldown,
}

impl BlockState for TrialSpawner {
    fn to_id(&self) -> i32 {
        if self.r#ominous == false
            && self.r#trial_spawner_state == TrialSpawnerState::EjectingReward
        {
            return 29465;
        }
        if self.r#ominous == false && self.r#trial_spawner_state == TrialSpawnerState::Inactive {
            return 29461;
        }
        if self.r#trial_spawner_state == TrialSpawnerState::WaitingForPlayers
            && self.r#ominous == true
        {
            return 29456;
        }
        if self.r#ominous == true && self.r#trial_spawner_state == TrialSpawnerState::EjectingReward
        {
            return 29459;
        }
        if self.r#trial_spawner_state == TrialSpawnerState::Cooldown && self.r#ominous == true {
            return 29460;
        }
        if self.r#trial_spawner_state == TrialSpawnerState::WaitingForRewardEjection
            && self.r#ominous == false
        {
            return 29464;
        }
        if self.r#ominous == true
            && self.r#trial_spawner_state == TrialSpawnerState::WaitingForRewardEjection
        {
            return 29458;
        }
        if self.r#ominous == true && self.r#trial_spawner_state == TrialSpawnerState::Active {
            return 29457;
        }
        if self.r#ominous == false
            && self.r#trial_spawner_state == TrialSpawnerState::WaitingForPlayers
        {
            return 29462;
        }
        if self.r#trial_spawner_state == TrialSpawnerState::Inactive && self.r#ominous == true {
            return 29455;
        }
        if self.r#ominous == false && self.r#trial_spawner_state == TrialSpawnerState::Cooldown {
            return 29466;
        }
        if self.r#ominous == false && self.r#trial_spawner_state == TrialSpawnerState::Active {
            return 29463;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29465 {
            return Some(TrialSpawner {
                r#ominous: false,
                r#trial_spawner_state: TrialSpawnerState::EjectingReward,
            });
        }
        if state_id == 29461 {
            return Some(TrialSpawner {
                r#ominous: false,
                r#trial_spawner_state: TrialSpawnerState::Inactive,
            });
        }
        if state_id == 29456 {
            return Some(TrialSpawner {
                r#trial_spawner_state: TrialSpawnerState::WaitingForPlayers,
                r#ominous: true,
            });
        }
        if state_id == 29459 {
            return Some(TrialSpawner {
                r#ominous: true,
                r#trial_spawner_state: TrialSpawnerState::EjectingReward,
            });
        }
        if state_id == 29460 {
            return Some(TrialSpawner {
                r#trial_spawner_state: TrialSpawnerState::Cooldown,
                r#ominous: true,
            });
        }
        if state_id == 29464 {
            return Some(TrialSpawner {
                r#trial_spawner_state: TrialSpawnerState::WaitingForRewardEjection,
                r#ominous: false,
            });
        }
        if state_id == 29458 {
            return Some(TrialSpawner {
                r#ominous: true,
                r#trial_spawner_state: TrialSpawnerState::WaitingForRewardEjection,
            });
        }
        if state_id == 29457 {
            return Some(TrialSpawner {
                r#ominous: true,
                r#trial_spawner_state: TrialSpawnerState::Active,
            });
        }
        if state_id == 29462 {
            return Some(TrialSpawner {
                r#ominous: false,
                r#trial_spawner_state: TrialSpawnerState::WaitingForPlayers,
            });
        }
        if state_id == 29455 {
            return Some(TrialSpawner {
                r#trial_spawner_state: TrialSpawnerState::Inactive,
                r#ominous: true,
            });
        }
        if state_id == 29466 {
            return Some(TrialSpawner {
                r#ominous: false,
                r#trial_spawner_state: TrialSpawnerState::Cooldown,
            });
        }
        if state_id == 29463 {
            return Some(TrialSpawner {
                r#ominous: false,
                r#trial_spawner_state: TrialSpawnerState::Active,
            });
        }
        return None;
    }
}
