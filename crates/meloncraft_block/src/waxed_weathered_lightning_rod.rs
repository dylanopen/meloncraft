use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredLightningRod {
    pub r#facing: Facing,
    pub powered: bool,
    pub waterlogged: bool,
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

impl BlockState for WaxedWeatheredLightningRod {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::Up && block_state.r#powered == true && block_state.r#waterlogged == false { return 27502; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#powered == false { return 27491; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#powered == true { return 27490; }
        if block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#facing == Facing::South { return 27494; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::Up && block_state.r#powered == false { return 27504; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::Down && block_state.r#powered == false { return 27508; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 27488; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 27487; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#powered == false { return 27495; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#powered == false { return 27500; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#powered == true { return 27493; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::Up { return 27501; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::East { return 27489; }
        if block_state.r#powered == true && block_state.r#facing == Facing::Down && block_state.r#waterlogged == true { return 27505; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 27492; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == true { return 27497; }
        if block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#facing == Facing::West { return 27499; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == false { return 27486; }
        if block_state.r#facing == Facing::Up && block_state.r#powered == false && block_state.r#waterlogged == true { return 27503; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#powered == false { return 27496; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 27498; }
        if block_state.r#facing == Facing::Down && block_state.r#powered == true && block_state.r#waterlogged == false { return 27506; }
        if block_state.r#facing == Facing::Down && block_state.r#powered == false && block_state.r#waterlogged == true { return 27507; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == true { return 27485; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27502 {
            return Some(WaxedWeatheredLightningRod {
                r#facing: Facing::Up,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 27491 {
            return Some(WaxedWeatheredLightningRod {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 27490 {
            return Some(WaxedWeatheredLightningRod {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 27494 {
            return Some(WaxedWeatheredLightningRod {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 27504 {
            return Some(WaxedWeatheredLightningRod {
                r#waterlogged: false,
                r#facing: Facing::Up,
                r#powered: false,
            });
        }
        if state_id == 27508 {
            return Some(WaxedWeatheredLightningRod {
                r#waterlogged: false,
                r#facing: Facing::Down,
                r#powered: false,
            });
        }
        if state_id == 27488 {
            return Some(WaxedWeatheredLightningRod {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 27487 {
            return Some(WaxedWeatheredLightningRod {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 27495 {
            return Some(WaxedWeatheredLightningRod {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 27500 {
            return Some(WaxedWeatheredLightningRod {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 27493 {
            return Some(WaxedWeatheredLightningRod {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 27501 {
            return Some(WaxedWeatheredLightningRod {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::Up,
            });
        }
        if state_id == 27489 {
            return Some(WaxedWeatheredLightningRod {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27505 {
            return Some(WaxedWeatheredLightningRod {
                r#powered: true,
                r#facing: Facing::Down,
                r#waterlogged: true,
            });
        }
        if state_id == 27492 {
            return Some(WaxedWeatheredLightningRod {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 27497 {
            return Some(WaxedWeatheredLightningRod {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27499 {
            return Some(WaxedWeatheredLightningRod {
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 27486 {
            return Some(WaxedWeatheredLightningRod {
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 27503 {
            return Some(WaxedWeatheredLightningRod {
                r#facing: Facing::Up,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27496 {
            return Some(WaxedWeatheredLightningRod {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 27498 {
            return Some(WaxedWeatheredLightningRod {
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 27506 {
            return Some(WaxedWeatheredLightningRod {
                r#facing: Facing::Down,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 27507 {
            return Some(WaxedWeatheredLightningRod {
                r#facing: Facing::Down,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27485 {
            return Some(WaxedWeatheredLightningRod {
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

