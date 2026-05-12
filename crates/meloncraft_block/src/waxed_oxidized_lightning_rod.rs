use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedOxidizedLightningRod {
    pub powered: bool,
    pub waterlogged: bool,
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

impl BlockState for WaxedOxidizedLightningRod {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#powered == false { return 27516; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 27511; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#waterlogged == true { return 27519; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#waterlogged == false { return 27524; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::Up { return 27525; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#powered == false { return 27523; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::Down { return 27529; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#powered == false { return 27520; }
        if block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#facing == Facing::South { return 27518; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == true { return 27521; }
        if block_state.r#facing == Facing::Up && block_state.r#powered == true && block_state.r#waterlogged == false { return 27526; }
        if block_state.r#powered == false && block_state.r#facing == Facing::Up && block_state.r#waterlogged == false { return 27528; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == false { return 27522; }
        if block_state.r#facing == Facing::Down && block_state.r#powered == true && block_state.r#waterlogged == false { return 27530; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#powered == true { return 27510; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#waterlogged == true { return 27513; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#powered == true { return 27514; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#powered == false { return 27515; }
        if block_state.r#facing == Facing::Down && block_state.r#waterlogged == false && block_state.r#powered == false { return 27532; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 27509; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#powered == false { return 27512; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 27517; }
        if block_state.r#powered == false && block_state.r#facing == Facing::Up && block_state.r#waterlogged == true { return 27527; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::Down { return 27531; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27516 {
            return Some(WaxedOxidizedLightningRod {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 27511 {
            return Some(WaxedOxidizedLightningRod {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 27519 {
            return Some(WaxedOxidizedLightningRod {
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27524 {
            return Some(WaxedOxidizedLightningRod {
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 27525 {
            return Some(WaxedOxidizedLightningRod {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::Up,
            });
        }
        if state_id == 27523 {
            return Some(WaxedOxidizedLightningRod {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 27529 {
            return Some(WaxedOxidizedLightningRod {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::Down,
            });
        }
        if state_id == 27520 {
            return Some(WaxedOxidizedLightningRod {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 27518 {
            return Some(WaxedOxidizedLightningRod {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 27521 {
            return Some(WaxedOxidizedLightningRod {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27526 {
            return Some(WaxedOxidizedLightningRod {
                r#facing: Facing::Up,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 27528 {
            return Some(WaxedOxidizedLightningRod {
                r#powered: false,
                r#facing: Facing::Up,
                r#waterlogged: false,
            });
        }
        if state_id == 27522 {
            return Some(WaxedOxidizedLightningRod {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 27530 {
            return Some(WaxedOxidizedLightningRod {
                r#facing: Facing::Down,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 27510 {
            return Some(WaxedOxidizedLightningRod {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 27513 {
            return Some(WaxedOxidizedLightningRod {
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27514 {
            return Some(WaxedOxidizedLightningRod {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 27515 {
            return Some(WaxedOxidizedLightningRod {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 27532 {
            return Some(WaxedOxidizedLightningRod {
                r#facing: Facing::Down,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 27509 {
            return Some(WaxedOxidizedLightningRod {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 27512 {
            return Some(WaxedOxidizedLightningRod {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 27517 {
            return Some(WaxedOxidizedLightningRod {
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 27527 {
            return Some(WaxedOxidizedLightningRod {
                r#powered: false,
                r#facing: Facing::Up,
                r#waterlogged: true,
            });
        }
        if state_id == 27531 {
            return Some(WaxedOxidizedLightningRod {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::Down,
            });
        }
        return None;
    }
}

