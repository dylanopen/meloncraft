use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredLightningRod {
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

impl BlockState for WaxedWeatheredLightningRod {
    fn to_id(&self) -> i32 {
        if self.r#powered == false && self.r#facing == Facing::West && self.r#waterlogged == true { return 27499; }
        if self.r#waterlogged == true && self.r#powered == true && self.r#facing == Facing::East { return 27489; }
        if self.r#facing == Facing::North && self.r#powered == false && self.r#waterlogged == true { return 27487; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#powered == false { return 27496; }
        if self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::East { return 27490; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#powered == false { return 27495; }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#waterlogged == true { return 27493; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#powered == false { return 27500; }
        if self.r#waterlogged == true && self.r#facing == Facing::Up && self.r#powered == true { return 27501; }
        if self.r#waterlogged == false && self.r#powered == true && self.r#facing == Facing::Up { return 27502; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#powered == false { return 27492; }
        if self.r#powered == false && self.r#facing == Facing::Up && self.r#waterlogged == false { return 27504; }
        if self.r#waterlogged == false && self.r#facing == Facing::Down && self.r#powered == true { return 27506; }
        if self.r#waterlogged == true && self.r#powered == true && self.r#facing == Facing::North { return 27485; }
        if self.r#powered == true && self.r#facing == Facing::West && self.r#waterlogged == false { return 27498; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#powered == true { return 27497; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#powered == true { return 27486; }
        if self.r#facing == Facing::Down && self.r#powered == true && self.r#waterlogged == true { return 27505; }
        if self.r#facing == Facing::Up && self.r#powered == false && self.r#waterlogged == true { return 27503; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#powered == false { return 27491; }
        if self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::North { return 27488; }
        if self.r#waterlogged == false && self.r#powered == false && self.r#facing == Facing::Down { return 27508; }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#waterlogged == false { return 27494; }
        if self.r#facing == Facing::Down && self.r#waterlogged == true && self.r#powered == false { return 27507; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27499 {
            return Some(WaxedWeatheredLightningRod {
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 27489 {
            return Some(WaxedWeatheredLightningRod {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27487 {
            return Some(WaxedWeatheredLightningRod {
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27496 {
            return Some(WaxedWeatheredLightningRod {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 27490 {
            return Some(WaxedWeatheredLightningRod {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 27495 {
            return Some(WaxedWeatheredLightningRod {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 27493 {
            return Some(WaxedWeatheredLightningRod {
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 27500 {
            return Some(WaxedWeatheredLightningRod {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 27501 {
            return Some(WaxedWeatheredLightningRod {
                r#waterlogged: true,
                r#facing: Facing::Up,
                r#powered: true,
            });
        }
        if state_id == 27502 {
            return Some(WaxedWeatheredLightningRod {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::Up,
            });
        }
        if state_id == 27492 {
            return Some(WaxedWeatheredLightningRod {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 27504 {
            return Some(WaxedWeatheredLightningRod {
                r#powered: false,
                r#facing: Facing::Up,
                r#waterlogged: false,
            });
        }
        if state_id == 27506 {
            return Some(WaxedWeatheredLightningRod {
                r#waterlogged: false,
                r#facing: Facing::Down,
                r#powered: true,
            });
        }
        if state_id == 27485 {
            return Some(WaxedWeatheredLightningRod {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 27498 {
            return Some(WaxedWeatheredLightningRod {
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 27497 {
            return Some(WaxedWeatheredLightningRod {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 27486 {
            return Some(WaxedWeatheredLightningRod {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 27505 {
            return Some(WaxedWeatheredLightningRod {
                r#facing: Facing::Down,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27503 {
            return Some(WaxedWeatheredLightningRod {
                r#facing: Facing::Up,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27491 {
            return Some(WaxedWeatheredLightningRod {
                r#waterlogged: true,
                r#facing: Facing::East,
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
        if state_id == 27508 {
            return Some(WaxedWeatheredLightningRod {
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::Down,
            });
        }
        if state_id == 27494 {
            return Some(WaxedWeatheredLightningRod {
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 27507 {
            return Some(WaxedWeatheredLightningRod {
                r#facing: Facing::Down,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        return None;
    }
}

