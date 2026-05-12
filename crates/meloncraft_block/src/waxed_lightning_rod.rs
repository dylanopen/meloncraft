use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedLightningRod {
    pub waterlogged: bool,
    pub r#facing: Facing,
    pub powered: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    East,
    South,
    West,
    Up,
    Down,
}

impl BlockState for WaxedLightningRod {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#powered == false { return 27444; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#powered == true { return 27446; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#powered == false { return 27452; }
        if block_state.r#powered == true && block_state.r#facing == Facing::Up && block_state.r#waterlogged == false { return 27454; }
        if block_state.r#facing == Facing::Down && block_state.r#powered == true && block_state.r#waterlogged == false { return 27458; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 27448; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 27447; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == true { return 27449; }
        if block_state.r#powered == false && block_state.r#facing == Facing::Down && block_state.r#waterlogged == true { return 27459; }
        if block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#facing == Facing::Up { return 27455; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == true { return 27437; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#powered == true { return 27441; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::Up && block_state.r#powered == false { return 27456; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 27442; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#waterlogged == true { return 27445; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#waterlogged == true { return 27443; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 27450; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::Down && block_state.r#powered == true { return 27457; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#powered == false { return 27439; }
        if block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#facing == Facing::West { return 27451; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#powered == false { return 27440; }
        if block_state.r#facing == Facing::Down && block_state.r#powered == false && block_state.r#waterlogged == false { return 27460; }
        if block_state.r#facing == Facing::Up && block_state.r#waterlogged == true && block_state.r#powered == true { return 27453; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 27438; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27444 {
            return Some(WaxedLightningRod {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 27446 {
            return Some(WaxedLightningRod {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 27452 {
            return Some(WaxedLightningRod {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 27454 {
            return Some(WaxedLightningRod {
                r#powered: true,
                r#facing: Facing::Up,
                r#waterlogged: false,
            });
        }
        if state_id == 27458 {
            return Some(WaxedLightningRod {
                r#facing: Facing::Down,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 27448 {
            return Some(WaxedLightningRod {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 27447 {
            return Some(WaxedLightningRod {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 27449 {
            return Some(WaxedLightningRod {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27459 {
            return Some(WaxedLightningRod {
                r#powered: false,
                r#facing: Facing::Down,
                r#waterlogged: true,
            });
        }
        if state_id == 27455 {
            return Some(WaxedLightningRod {
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::Up,
            });
        }
        if state_id == 27437 {
            return Some(WaxedLightningRod {
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27441 {
            return Some(WaxedLightningRod {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 27456 {
            return Some(WaxedLightningRod {
                r#waterlogged: false,
                r#facing: Facing::Up,
                r#powered: false,
            });
        }
        if state_id == 27442 {
            return Some(WaxedLightningRod {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 27445 {
            return Some(WaxedLightningRod {
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27443 {
            return Some(WaxedLightningRod {
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27450 {
            return Some(WaxedLightningRod {
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 27457 {
            return Some(WaxedLightningRod {
                r#waterlogged: true,
                r#facing: Facing::Down,
                r#powered: true,
            });
        }
        if state_id == 27439 {
            return Some(WaxedLightningRod {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 27451 {
            return Some(WaxedLightningRod {
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 27440 {
            return Some(WaxedLightningRod {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 27460 {
            return Some(WaxedLightningRod {
                r#facing: Facing::Down,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 27453 {
            return Some(WaxedLightningRod {
                r#facing: Facing::Up,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 27438 {
            return Some(WaxedLightningRod {
                r#powered: true,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

