use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vault {
    pub r#facing: Facing,
    pub ominous: bool,
    pub r#vault_state: VaultState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VaultState {
    Inactive,
    Active,
    Unlocking,
    Ejecting,
}

impl BlockState for Vault {
    fn to_id(&self) -> i32 {
        if self.r#vault_state == VaultState::Unlocking
            && self.r#ominous == true
            && self.r#facing == Facing::South
        {
            return 29477;
        }
        if self.r#facing == Facing::West
            && self.r#ominous == true
            && self.r#vault_state == VaultState::Unlocking
        {
            return 29485;
        }
        if self.r#ominous == false
            && self.r#vault_state == VaultState::Inactive
            && self.r#facing == Facing::West
        {
            return 29487;
        }
        if self.r#ominous == false
            && self.r#vault_state == VaultState::Inactive
            && self.r#facing == Facing::North
        {
            return 29471;
        }
        if self.r#vault_state == VaultState::Active
            && self.r#facing == Facing::West
            && self.r#ominous == false
        {
            return 29488;
        }
        if self.r#facing == Facing::East
            && self.r#ominous == false
            && self.r#vault_state == VaultState::Inactive
        {
            return 29495;
        }
        if self.r#ominous == true
            && self.r#facing == Facing::South
            && self.r#vault_state == VaultState::Inactive
        {
            return 29475;
        }
        if self.r#vault_state == VaultState::Ejecting
            && self.r#facing == Facing::South
            && self.r#ominous == true
        {
            return 29478;
        }
        if self.r#facing == Facing::West
            && self.r#vault_state == VaultState::Unlocking
            && self.r#ominous == false
        {
            return 29489;
        }
        if self.r#vault_state == VaultState::Unlocking
            && self.r#ominous == false
            && self.r#facing == Facing::South
        {
            return 29481;
        }
        if self.r#vault_state == VaultState::Ejecting
            && self.r#ominous == true
            && self.r#facing == Facing::East
        {
            return 29494;
        }
        if self.r#ominous == false
            && self.r#facing == Facing::East
            && self.r#vault_state == VaultState::Ejecting
        {
            return 29498;
        }
        if self.r#facing == Facing::South
            && self.r#ominous == false
            && self.r#vault_state == VaultState::Inactive
        {
            return 29479;
        }
        if self.r#ominous == false
            && self.r#facing == Facing::South
            && self.r#vault_state == VaultState::Ejecting
        {
            return 29482;
        }
        if self.r#facing == Facing::West
            && self.r#ominous == true
            && self.r#vault_state == VaultState::Inactive
        {
            return 29483;
        }
        if self.r#ominous == true
            && self.r#vault_state == VaultState::Unlocking
            && self.r#facing == Facing::East
        {
            return 29493;
        }
        if self.r#ominous == false
            && self.r#vault_state == VaultState::Unlocking
            && self.r#facing == Facing::East
        {
            return 29497;
        }
        if self.r#ominous == true
            && self.r#vault_state == VaultState::Active
            && self.r#facing == Facing::East
        {
            return 29492;
        }
        if self.r#vault_state == VaultState::Active
            && self.r#ominous == true
            && self.r#facing == Facing::West
        {
            return 29484;
        }
        if self.r#vault_state == VaultState::Active
            && self.r#facing == Facing::South
            && self.r#ominous == false
        {
            return 29480;
        }
        if self.r#ominous == false
            && self.r#vault_state == VaultState::Active
            && self.r#facing == Facing::North
        {
            return 29472;
        }
        if self.r#facing == Facing::West
            && self.r#vault_state == VaultState::Ejecting
            && self.r#ominous == true
        {
            return 29486;
        }
        if self.r#facing == Facing::East
            && self.r#ominous == true
            && self.r#vault_state == VaultState::Inactive
        {
            return 29491;
        }
        if self.r#ominous == true
            && self.r#facing == Facing::South
            && self.r#vault_state == VaultState::Active
        {
            return 29476;
        }
        if self.r#ominous == false
            && self.r#facing == Facing::West
            && self.r#vault_state == VaultState::Ejecting
        {
            return 29490;
        }
        if self.r#ominous == false
            && self.r#facing == Facing::North
            && self.r#vault_state == VaultState::Unlocking
        {
            return 29473;
        }
        if self.r#ominous == true
            && self.r#vault_state == VaultState::Unlocking
            && self.r#facing == Facing::North
        {
            return 29469;
        }
        if self.r#vault_state == VaultState::Ejecting
            && self.r#facing == Facing::North
            && self.r#ominous == false
        {
            return 29474;
        }
        if self.r#ominous == true
            && self.r#vault_state == VaultState::Ejecting
            && self.r#facing == Facing::North
        {
            return 29470;
        }
        if self.r#ominous == true
            && self.r#facing == Facing::North
            && self.r#vault_state == VaultState::Inactive
        {
            return 29467;
        }
        if self.r#ominous == true
            && self.r#vault_state == VaultState::Active
            && self.r#facing == Facing::North
        {
            return 29468;
        }
        if self.r#ominous == false
            && self.r#vault_state == VaultState::Active
            && self.r#facing == Facing::East
        {
            return 29496;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29477 {
            return Some(Vault {
                r#vault_state: VaultState::Unlocking,
                r#ominous: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 29485 {
            return Some(Vault {
                r#facing: Facing::West,
                r#ominous: true,
                r#vault_state: VaultState::Unlocking,
            });
        }
        if state_id == 29487 {
            return Some(Vault {
                r#ominous: false,
                r#vault_state: VaultState::Inactive,
                r#facing: Facing::West,
            });
        }
        if state_id == 29471 {
            return Some(Vault {
                r#ominous: false,
                r#vault_state: VaultState::Inactive,
                r#facing: Facing::North,
            });
        }
        if state_id == 29488 {
            return Some(Vault {
                r#vault_state: VaultState::Active,
                r#facing: Facing::West,
                r#ominous: false,
            });
        }
        if state_id == 29495 {
            return Some(Vault {
                r#facing: Facing::East,
                r#ominous: false,
                r#vault_state: VaultState::Inactive,
            });
        }
        if state_id == 29475 {
            return Some(Vault {
                r#ominous: true,
                r#facing: Facing::South,
                r#vault_state: VaultState::Inactive,
            });
        }
        if state_id == 29478 {
            return Some(Vault {
                r#vault_state: VaultState::Ejecting,
                r#facing: Facing::South,
                r#ominous: true,
            });
        }
        if state_id == 29489 {
            return Some(Vault {
                r#facing: Facing::West,
                r#vault_state: VaultState::Unlocking,
                r#ominous: false,
            });
        }
        if state_id == 29481 {
            return Some(Vault {
                r#vault_state: VaultState::Unlocking,
                r#ominous: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 29494 {
            return Some(Vault {
                r#vault_state: VaultState::Ejecting,
                r#ominous: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 29498 {
            return Some(Vault {
                r#ominous: false,
                r#facing: Facing::East,
                r#vault_state: VaultState::Ejecting,
            });
        }
        if state_id == 29479 {
            return Some(Vault {
                r#facing: Facing::South,
                r#ominous: false,
                r#vault_state: VaultState::Inactive,
            });
        }
        if state_id == 29482 {
            return Some(Vault {
                r#ominous: false,
                r#facing: Facing::South,
                r#vault_state: VaultState::Ejecting,
            });
        }
        if state_id == 29483 {
            return Some(Vault {
                r#facing: Facing::West,
                r#ominous: true,
                r#vault_state: VaultState::Inactive,
            });
        }
        if state_id == 29493 {
            return Some(Vault {
                r#ominous: true,
                r#vault_state: VaultState::Unlocking,
                r#facing: Facing::East,
            });
        }
        if state_id == 29497 {
            return Some(Vault {
                r#ominous: false,
                r#vault_state: VaultState::Unlocking,
                r#facing: Facing::East,
            });
        }
        if state_id == 29492 {
            return Some(Vault {
                r#ominous: true,
                r#vault_state: VaultState::Active,
                r#facing: Facing::East,
            });
        }
        if state_id == 29484 {
            return Some(Vault {
                r#vault_state: VaultState::Active,
                r#ominous: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 29480 {
            return Some(Vault {
                r#vault_state: VaultState::Active,
                r#facing: Facing::South,
                r#ominous: false,
            });
        }
        if state_id == 29472 {
            return Some(Vault {
                r#ominous: false,
                r#vault_state: VaultState::Active,
                r#facing: Facing::North,
            });
        }
        if state_id == 29486 {
            return Some(Vault {
                r#facing: Facing::West,
                r#vault_state: VaultState::Ejecting,
                r#ominous: true,
            });
        }
        if state_id == 29491 {
            return Some(Vault {
                r#facing: Facing::East,
                r#ominous: true,
                r#vault_state: VaultState::Inactive,
            });
        }
        if state_id == 29476 {
            return Some(Vault {
                r#ominous: true,
                r#facing: Facing::South,
                r#vault_state: VaultState::Active,
            });
        }
        if state_id == 29490 {
            return Some(Vault {
                r#ominous: false,
                r#facing: Facing::West,
                r#vault_state: VaultState::Ejecting,
            });
        }
        if state_id == 29473 {
            return Some(Vault {
                r#ominous: false,
                r#facing: Facing::North,
                r#vault_state: VaultState::Unlocking,
            });
        }
        if state_id == 29469 {
            return Some(Vault {
                r#ominous: true,
                r#vault_state: VaultState::Unlocking,
                r#facing: Facing::North,
            });
        }
        if state_id == 29474 {
            return Some(Vault {
                r#vault_state: VaultState::Ejecting,
                r#facing: Facing::North,
                r#ominous: false,
            });
        }
        if state_id == 29470 {
            return Some(Vault {
                r#ominous: true,
                r#vault_state: VaultState::Ejecting,
                r#facing: Facing::North,
            });
        }
        if state_id == 29467 {
            return Some(Vault {
                r#ominous: true,
                r#facing: Facing::North,
                r#vault_state: VaultState::Inactive,
            });
        }
        if state_id == 29468 {
            return Some(Vault {
                r#ominous: true,
                r#vault_state: VaultState::Active,
                r#facing: Facing::North,
            });
        }
        if state_id == 29496 {
            return Some(Vault {
                r#ominous: false,
                r#vault_state: VaultState::Active,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}
