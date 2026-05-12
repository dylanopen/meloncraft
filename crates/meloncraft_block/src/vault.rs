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
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::East && block_state.r#ominous == true && block_state.r#vault_state == VaultState::Inactive { return 29491; }
        if block_state.r#ominous == true && block_state.r#vault_state == VaultState::Unlocking && block_state.r#facing == Facing::East { return 29493; }
        if block_state.r#facing == Facing::North && block_state.r#ominous == false && block_state.r#vault_state == VaultState::Ejecting { return 29474; }
        if block_state.r#facing == Facing::South && block_state.r#ominous == true && block_state.r#vault_state == VaultState::Ejecting { return 29478; }
        if block_state.r#ominous == false && block_state.r#facing == Facing::South && block_state.r#vault_state == VaultState::Unlocking { return 29481; }
        if block_state.r#vault_state == VaultState::Unlocking && block_state.r#facing == Facing::North && block_state.r#ominous == false { return 29473; }
        if block_state.r#ominous == true && block_state.r#vault_state == VaultState::Active && block_state.r#facing == Facing::West { return 29484; }
        if block_state.r#vault_state == VaultState::Active && block_state.r#ominous == true && block_state.r#facing == Facing::East { return 29492; }
        if block_state.r#facing == Facing::South && block_state.r#ominous == true && block_state.r#vault_state == VaultState::Active { return 29476; }
        if block_state.r#facing == Facing::South && block_state.r#ominous == true && block_state.r#vault_state == VaultState::Unlocking { return 29477; }
        if block_state.r#vault_state == VaultState::Inactive && block_state.r#ominous == true && block_state.r#facing == Facing::South { return 29475; }
        if block_state.r#ominous == true && block_state.r#facing == Facing::North && block_state.r#vault_state == VaultState::Active { return 29468; }
        if block_state.r#ominous == false && block_state.r#vault_state == VaultState::Active && block_state.r#facing == Facing::South { return 29480; }
        if block_state.r#facing == Facing::South && block_state.r#vault_state == VaultState::Ejecting && block_state.r#ominous == false { return 29482; }
        if block_state.r#facing == Facing::East && block_state.r#ominous == false && block_state.r#vault_state == VaultState::Inactive { return 29495; }
        if block_state.r#ominous == false && block_state.r#vault_state == VaultState::Active && block_state.r#facing == Facing::West { return 29488; }
        if block_state.r#facing == Facing::East && block_state.r#ominous == false && block_state.r#vault_state == VaultState::Ejecting { return 29498; }
        if block_state.r#facing == Facing::West && block_state.r#ominous == false && block_state.r#vault_state == VaultState::Ejecting { return 29490; }
        if block_state.r#facing == Facing::West && block_state.r#vault_state == VaultState::Inactive && block_state.r#ominous == true { return 29483; }
        if block_state.r#ominous == false && block_state.r#vault_state == VaultState::Active && block_state.r#facing == Facing::East { return 29496; }
        if block_state.r#facing == Facing::North && block_state.r#ominous == true && block_state.r#vault_state == VaultState::Unlocking { return 29469; }
        if block_state.r#ominous == true && block_state.r#facing == Facing::North && block_state.r#vault_state == VaultState::Ejecting { return 29470; }
        if block_state.r#vault_state == VaultState::Inactive && block_state.r#facing == Facing::West && block_state.r#ominous == false { return 29487; }
        if block_state.r#ominous == true && block_state.r#vault_state == VaultState::Ejecting && block_state.r#facing == Facing::West { return 29486; }
        if block_state.r#facing == Facing::East && block_state.r#ominous == false && block_state.r#vault_state == VaultState::Unlocking { return 29497; }
        if block_state.r#facing == Facing::North && block_state.r#ominous == false && block_state.r#vault_state == VaultState::Active { return 29472; }
        if block_state.r#vault_state == VaultState::Unlocking && block_state.r#facing == Facing::West && block_state.r#ominous == false { return 29489; }
        if block_state.r#vault_state == VaultState::Inactive && block_state.r#facing == Facing::North && block_state.r#ominous == true { return 29467; }
        if block_state.r#facing == Facing::South && block_state.r#ominous == false && block_state.r#vault_state == VaultState::Inactive { return 29479; }
        if block_state.r#ominous == false && block_state.r#facing == Facing::North && block_state.r#vault_state == VaultState::Inactive { return 29471; }
        if block_state.r#vault_state == VaultState::Unlocking && block_state.r#ominous == true && block_state.r#facing == Facing::West { return 29485; }
        if block_state.r#vault_state == VaultState::Ejecting && block_state.r#facing == Facing::East && block_state.r#ominous == true { return 29494; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29491 {
            return Some(Vault {
                r#facing: Facing::East,
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
        if state_id == 29474 {
            return Some(Vault {
                r#facing: Facing::North,
                r#ominous: false,
                r#vault_state: VaultState::Ejecting,
            });
        }
        if state_id == 29478 {
            return Some(Vault {
                r#facing: Facing::South,
                r#ominous: true,
                r#vault_state: VaultState::Ejecting,
            });
        }
        if state_id == 29481 {
            return Some(Vault {
                r#ominous: false,
                r#facing: Facing::South,
                r#vault_state: VaultState::Unlocking,
            });
        }
        if state_id == 29473 {
            return Some(Vault {
                r#vault_state: VaultState::Unlocking,
                r#facing: Facing::North,
                r#ominous: false,
            });
        }
        if state_id == 29484 {
            return Some(Vault {
                r#ominous: true,
                r#vault_state: VaultState::Active,
                r#facing: Facing::West,
            });
        }
        if state_id == 29492 {
            return Some(Vault {
                r#vault_state: VaultState::Active,
                r#ominous: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 29476 {
            return Some(Vault {
                r#facing: Facing::South,
                r#ominous: true,
                r#vault_state: VaultState::Active,
            });
        }
        if state_id == 29477 {
            return Some(Vault {
                r#facing: Facing::South,
                r#ominous: true,
                r#vault_state: VaultState::Unlocking,
            });
        }
        if state_id == 29475 {
            return Some(Vault {
                r#vault_state: VaultState::Inactive,
                r#ominous: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 29468 {
            return Some(Vault {
                r#ominous: true,
                r#facing: Facing::North,
                r#vault_state: VaultState::Active,
            });
        }
        if state_id == 29480 {
            return Some(Vault {
                r#ominous: false,
                r#vault_state: VaultState::Active,
                r#facing: Facing::South,
            });
        }
        if state_id == 29482 {
            return Some(Vault {
                r#facing: Facing::South,
                r#vault_state: VaultState::Ejecting,
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
        if state_id == 29488 {
            return Some(Vault {
                r#ominous: false,
                r#vault_state: VaultState::Active,
                r#facing: Facing::West,
            });
        }
        if state_id == 29498 {
            return Some(Vault {
                r#facing: Facing::East,
                r#ominous: false,
                r#vault_state: VaultState::Ejecting,
            });
        }
        if state_id == 29490 {
            return Some(Vault {
                r#facing: Facing::West,
                r#ominous: false,
                r#vault_state: VaultState::Ejecting,
            });
        }
        if state_id == 29483 {
            return Some(Vault {
                r#facing: Facing::West,
                r#vault_state: VaultState::Inactive,
                r#ominous: true,
            });
        }
        if state_id == 29496 {
            return Some(Vault {
                r#ominous: false,
                r#vault_state: VaultState::Active,
                r#facing: Facing::East,
            });
        }
        if state_id == 29469 {
            return Some(Vault {
                r#facing: Facing::North,
                r#ominous: true,
                r#vault_state: VaultState::Unlocking,
            });
        }
        if state_id == 29470 {
            return Some(Vault {
                r#ominous: true,
                r#facing: Facing::North,
                r#vault_state: VaultState::Ejecting,
            });
        }
        if state_id == 29487 {
            return Some(Vault {
                r#vault_state: VaultState::Inactive,
                r#facing: Facing::West,
                r#ominous: false,
            });
        }
        if state_id == 29486 {
            return Some(Vault {
                r#ominous: true,
                r#vault_state: VaultState::Ejecting,
                r#facing: Facing::West,
            });
        }
        if state_id == 29497 {
            return Some(Vault {
                r#facing: Facing::East,
                r#ominous: false,
                r#vault_state: VaultState::Unlocking,
            });
        }
        if state_id == 29472 {
            return Some(Vault {
                r#facing: Facing::North,
                r#ominous: false,
                r#vault_state: VaultState::Active,
            });
        }
        if state_id == 29489 {
            return Some(Vault {
                r#vault_state: VaultState::Unlocking,
                r#facing: Facing::West,
                r#ominous: false,
            });
        }
        if state_id == 29467 {
            return Some(Vault {
                r#vault_state: VaultState::Inactive,
                r#facing: Facing::North,
                r#ominous: true,
            });
        }
        if state_id == 29479 {
            return Some(Vault {
                r#facing: Facing::South,
                r#ominous: false,
                r#vault_state: VaultState::Inactive,
            });
        }
        if state_id == 29471 {
            return Some(Vault {
                r#ominous: false,
                r#facing: Facing::North,
                r#vault_state: VaultState::Inactive,
            });
        }
        if state_id == 29485 {
            return Some(Vault {
                r#vault_state: VaultState::Unlocking,
                r#ominous: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 29494 {
            return Some(Vault {
                r#vault_state: VaultState::Ejecting,
                r#facing: Facing::East,
                r#ominous: true,
            });
        }
        return None;
    }
}

