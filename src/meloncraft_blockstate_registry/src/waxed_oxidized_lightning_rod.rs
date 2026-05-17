use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedOxidizedLightningRod {
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

impl BlockState for WaxedOxidizedLightningRod {
    fn to_id(&self) -> i32 {
        if self.r#powered == true && self.r#waterlogged == true && self.r#facing == Facing::Down { return 27529; }
        if self.r#facing == Facing::South && self.r#powered == false && self.r#waterlogged == false { return 27520; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#powered == true { return 27509; }
        if self.r#waterlogged == true && self.r#facing == Facing::Up && self.r#powered == true { return 27525; }
        if self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::Up { return 27528; }
        if self.r#waterlogged == false && self.r#facing == Facing::Down && self.r#powered == true { return 27530; }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#waterlogged == false { return 27518; }
        if self.r#facing == Facing::Down && self.r#waterlogged == true && self.r#powered == false { return 27531; }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#waterlogged == true { return 27515; }
        if self.r#powered == false && self.r#facing == Facing::North && self.r#waterlogged == true { return 27511; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#powered == true { return 27513; }
        if self.r#facing == Facing::Up && self.r#waterlogged == false && self.r#powered == true { return 27526; }
        if self.r#waterlogged == true && self.r#powered == true && self.r#facing == Facing::West { return 27521; }
        if self.r#powered == false && self.r#facing == Facing::West && self.r#waterlogged == true { return 27523; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#waterlogged == false { return 27522; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#powered == false { return 27524; }
        if self.r#waterlogged == false && self.r#powered == true && self.r#facing == Facing::North { return 27510; }
        if self.r#facing == Facing::East && self.r#powered == true && self.r#waterlogged == false { return 27514; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#powered == true { return 27517; }
        if self.r#powered == false && self.r#facing == Facing::East && self.r#waterlogged == false { return 27516; }
        if self.r#powered == false && self.r#waterlogged == true && self.r#facing == Facing::South { return 27519; }
        if self.r#powered == false && self.r#facing == Facing::Up && self.r#waterlogged == true { return 27527; }
        if self.r#waterlogged == false && self.r#facing == Facing::Down && self.r#powered == false { return 27532; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#powered == false { return 27512; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27529 {
            return Some(WaxedOxidizedLightningRod {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::Down,
            });
        }
        if state_id == 27520 {
            return Some(WaxedOxidizedLightningRod {
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 27509 {
            return Some(WaxedOxidizedLightningRod {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 27525 {
            return Some(WaxedOxidizedLightningRod {
                r#waterlogged: true,
                r#facing: Facing::Up,
                r#powered: true,
            });
        }
        if state_id == 27528 {
            return Some(WaxedOxidizedLightningRod {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::Up,
            });
        }
        if state_id == 27530 {
            return Some(WaxedOxidizedLightningRod {
                r#waterlogged: false,
                r#facing: Facing::Down,
                r#powered: true,
            });
        }
        if state_id == 27518 {
            return Some(WaxedOxidizedLightningRod {
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 27531 {
            return Some(WaxedOxidizedLightningRod {
                r#facing: Facing::Down,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 27515 {
            return Some(WaxedOxidizedLightningRod {
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27511 {
            return Some(WaxedOxidizedLightningRod {
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 27513 {
            return Some(WaxedOxidizedLightningRod {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 27526 {
            return Some(WaxedOxidizedLightningRod {
                r#facing: Facing::Up,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 27521 {
            return Some(WaxedOxidizedLightningRod {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 27523 {
            return Some(WaxedOxidizedLightningRod {
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 27522 {
            return Some(WaxedOxidizedLightningRod {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 27524 {
            return Some(WaxedOxidizedLightningRod {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 27510 {
            return Some(WaxedOxidizedLightningRod {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 27514 {
            return Some(WaxedOxidizedLightningRod {
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 27517 {
            return Some(WaxedOxidizedLightningRod {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 27516 {
            return Some(WaxedOxidizedLightningRod {
                r#powered: false,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27519 {
            return Some(WaxedOxidizedLightningRod {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 27527 {
            return Some(WaxedOxidizedLightningRod {
                r#powered: false,
                r#facing: Facing::Up,
                r#waterlogged: true,
            });
        }
        if state_id == 27532 {
            return Some(WaxedOxidizedLightningRod {
                r#waterlogged: false,
                r#facing: Facing::Down,
                r#powered: false,
            });
        }
        if state_id == 27512 {
            return Some(WaxedOxidizedLightningRod {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        return None;
    }
}

