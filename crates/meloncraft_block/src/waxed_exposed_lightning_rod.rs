use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedExposedLightningRod {
    pub waterlogged: bool,
    pub powered: bool,
    pub r#facing: Facing,
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

impl BlockState for WaxedExposedLightningRod {
    fn to_id(self) -> i32 {
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::Up { return 27478; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#powered == false { return 27468; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#waterlogged == true { return 27467; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#waterlogged == true { return 27469; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#facing == Facing::North { return 27464; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 27470; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#waterlogged == false { return 27472; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#powered == true { return 27465; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#powered == false { return 27471; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#powered == true { return 27473; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == true { return 27461; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::Down { return 27481; }
        if block_state.r#powered == false && block_state.r#facing == Facing::Down && block_state.r#waterlogged == true { return 27483; }
        if block_state.r#facing == Facing::Down && block_state.r#powered == false && block_state.r#waterlogged == false { return 27484; }
        if block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#facing == Facing::West { return 27475; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 27466; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#powered == false { return 27463; }
        if block_state.r#powered == true && block_state.r#facing == Facing::Down && block_state.r#waterlogged == false { return 27482; }
        if block_state.r#powered == false && block_state.r#facing == Facing::Up && block_state.r#waterlogged == true { return 27479; }
        if block_state.r#powered == false && block_state.r#facing == Facing::Up && block_state.r#waterlogged == false { return 27480; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#powered == true { return 27462; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 27476; }
        if block_state.r#facing == Facing::Up && block_state.r#powered == true && block_state.r#waterlogged == true { return 27477; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == false { return 27474; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27478 {
            return Some(WaxedExposedLightningRod {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::Up,
            });
        }
        if state_id == 27468 {
            return Some(WaxedExposedLightningRod {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 27467 {
            return Some(WaxedExposedLightningRod {
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27469 {
            return Some(WaxedExposedLightningRod {
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27464 {
            return Some(WaxedExposedLightningRod {
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 27470 {
            return Some(WaxedExposedLightningRod {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 27472 {
            return Some(WaxedExposedLightningRod {
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 27465 {
            return Some(WaxedExposedLightningRod {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 27471 {
            return Some(WaxedExposedLightningRod {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 27473 {
            return Some(WaxedExposedLightningRod {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 27461 {
            return Some(WaxedExposedLightningRod {
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27481 {
            return Some(WaxedExposedLightningRod {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::Down,
            });
        }
        if state_id == 27483 {
            return Some(WaxedExposedLightningRod {
                r#powered: false,
                r#facing: Facing::Down,
                r#waterlogged: true,
            });
        }
        if state_id == 27484 {
            return Some(WaxedExposedLightningRod {
                r#facing: Facing::Down,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 27475 {
            return Some(WaxedExposedLightningRod {
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 27466 {
            return Some(WaxedExposedLightningRod {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 27463 {
            return Some(WaxedExposedLightningRod {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 27482 {
            return Some(WaxedExposedLightningRod {
                r#powered: true,
                r#facing: Facing::Down,
                r#waterlogged: false,
            });
        }
        if state_id == 27479 {
            return Some(WaxedExposedLightningRod {
                r#powered: false,
                r#facing: Facing::Up,
                r#waterlogged: true,
            });
        }
        if state_id == 27480 {
            return Some(WaxedExposedLightningRod {
                r#powered: false,
                r#facing: Facing::Up,
                r#waterlogged: false,
            });
        }
        if state_id == 27462 {
            return Some(WaxedExposedLightningRod {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 27476 {
            return Some(WaxedExposedLightningRod {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 27477 {
            return Some(WaxedExposedLightningRod {
                r#facing: Facing::Up,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27474 {
            return Some(WaxedExposedLightningRod {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

