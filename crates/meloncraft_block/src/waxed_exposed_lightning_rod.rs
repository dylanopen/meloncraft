use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedExposedLightningRod {
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

impl BlockState for WaxedExposedLightningRod {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::Down && self.r#powered == true && self.r#waterlogged == true { return 27481; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#powered == true { return 27461; }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#waterlogged == true { return 27469; }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#waterlogged == false { return 27468; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#waterlogged == true { return 27471; }
        if self.r#facing == Facing::Up && self.r#powered == true && self.r#waterlogged == true { return 27477; }
        if self.r#waterlogged == false && self.r#powered == true && self.r#facing == Facing::Down { return 27482; }
        if self.r#facing == Facing::Down && self.r#waterlogged == true && self.r#powered == false { return 27483; }
        if self.r#powered == true && self.r#facing == Facing::North && self.r#waterlogged == false { return 27462; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#powered == true { return 27473; }
        if self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::East { return 27466; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#powered == true { return 27474; }
        if self.r#facing == Facing::Up && self.r#powered == true && self.r#waterlogged == false { return 27478; }
        if self.r#powered == false && self.r#waterlogged == true && self.r#facing == Facing::East { return 27467; }
        if self.r#waterlogged == false && self.r#powered == false && self.r#facing == Facing::South { return 27472; }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#waterlogged == false { return 27470; }
        if self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::West { return 27476; }
        if self.r#facing == Facing::Down && self.r#waterlogged == false && self.r#powered == false { return 27484; }
        if self.r#facing == Facing::West && self.r#powered == false && self.r#waterlogged == true { return 27475; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#powered == false { return 27464; }
        if self.r#facing == Facing::North && self.r#powered == false && self.r#waterlogged == true { return 27463; }
        if self.r#facing == Facing::East && self.r#powered == true && self.r#waterlogged == true { return 27465; }
        if self.r#facing == Facing::Up && self.r#powered == false && self.r#waterlogged == true { return 27479; }
        if self.r#powered == false && self.r#facing == Facing::Up && self.r#waterlogged == false { return 27480; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27481 {
            return Some(WaxedExposedLightningRod {
                r#facing: Facing::Down,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27461 {
            return Some(WaxedExposedLightningRod {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 27469 {
            return Some(WaxedExposedLightningRod {
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 27468 {
            return Some(WaxedExposedLightningRod {
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 27471 {
            return Some(WaxedExposedLightningRod {
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 27477 {
            return Some(WaxedExposedLightningRod {
                r#facing: Facing::Up,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27482 {
            return Some(WaxedExposedLightningRod {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::Down,
            });
        }
        if state_id == 27483 {
            return Some(WaxedExposedLightningRod {
                r#facing: Facing::Down,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 27462 {
            return Some(WaxedExposedLightningRod {
                r#powered: true,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 27473 {
            return Some(WaxedExposedLightningRod {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 27466 {
            return Some(WaxedExposedLightningRod {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 27474 {
            return Some(WaxedExposedLightningRod {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 27478 {
            return Some(WaxedExposedLightningRod {
                r#facing: Facing::Up,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 27467 {
            return Some(WaxedExposedLightningRod {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27472 {
            return Some(WaxedExposedLightningRod {
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 27470 {
            return Some(WaxedExposedLightningRod {
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 27476 {
            return Some(WaxedExposedLightningRod {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 27484 {
            return Some(WaxedExposedLightningRod {
                r#facing: Facing::Down,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 27475 {
            return Some(WaxedExposedLightningRod {
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27464 {
            return Some(WaxedExposedLightningRod {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 27463 {
            return Some(WaxedExposedLightningRod {
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27465 {
            return Some(WaxedExposedLightningRod {
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27479 {
            return Some(WaxedExposedLightningRod {
                r#facing: Facing::Up,
                r#powered: false,
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
        return None;
    }
}

