use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedLightningRod {
    pub r#facing: Facing,
    pub waterlogged: bool,
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
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#powered == true && self.r#facing == Facing::West {
            return 27450;
        }
        if self.r#facing == Facing::Up && self.r#powered == true && self.r#waterlogged == false {
            return 27454;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#powered == true {
            return 27449;
        }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#powered == false {
            return 27451;
        }
        if self.r#waterlogged == false && self.r#powered == false && self.r#facing == Facing::Down {
            return 27460;
        }
        if self.r#powered == false && self.r#facing == Facing::West && self.r#waterlogged == false {
            return 27452;
        }
        if self.r#powered == false && self.r#waterlogged == true && self.r#facing == Facing::Up {
            return 27455;
        }
        if self.r#waterlogged == true && self.r#powered == true && self.r#facing == Facing::North {
            return 27437;
        }
        if self.r#facing == Facing::Down && self.r#powered == true && self.r#waterlogged == true {
            return 27457;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::Down && self.r#powered == true {
            return 27458;
        }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#powered == false {
            return 27439;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#powered == true {
            return 27441;
        }
        if self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::North
        {
            return 27440;
        }
        if self.r#powered == true && self.r#facing == Facing::Up && self.r#waterlogged == true {
            return 27453;
        }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#waterlogged == false {
            return 27446;
        }
        if self.r#waterlogged == false && self.r#powered == false && self.r#facing == Facing::Up {
            return 27456;
        }
        if self.r#powered == false && self.r#waterlogged == true && self.r#facing == Facing::East {
            return 27443;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#powered == true {
            return 27438;
        }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#powered == true {
            return 27445;
        }
        if self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::South
        {
            return 27448;
        }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#powered == true {
            return 27442;
        }
        if self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::East {
            return 27444;
        }
        if self.r#waterlogged == true && self.r#powered == false && self.r#facing == Facing::Down {
            return 27459;
        }
        if self.r#facing == Facing::South && self.r#powered == false && self.r#waterlogged == true {
            return 27447;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27450 {
            return Some(WaxedLightningRod {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 27454 {
            return Some(WaxedLightningRod {
                r#facing: Facing::Up,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 27449 {
            return Some(WaxedLightningRod {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 27451 {
            return Some(WaxedLightningRod {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 27460 {
            return Some(WaxedLightningRod {
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::Down,
            });
        }
        if state_id == 27452 {
            return Some(WaxedLightningRod {
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 27455 {
            return Some(WaxedLightningRod {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::Up,
            });
        }
        if state_id == 27437 {
            return Some(WaxedLightningRod {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 27457 {
            return Some(WaxedLightningRod {
                r#facing: Facing::Down,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27458 {
            return Some(WaxedLightningRod {
                r#waterlogged: false,
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
        if state_id == 27441 {
            return Some(WaxedLightningRod {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 27440 {
            return Some(WaxedLightningRod {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 27453 {
            return Some(WaxedLightningRod {
                r#powered: true,
                r#facing: Facing::Up,
                r#waterlogged: true,
            });
        }
        if state_id == 27446 {
            return Some(WaxedLightningRod {
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 27456 {
            return Some(WaxedLightningRod {
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::Up,
            });
        }
        if state_id == 27443 {
            return Some(WaxedLightningRod {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27438 {
            return Some(WaxedLightningRod {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 27445 {
            return Some(WaxedLightningRod {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 27448 {
            return Some(WaxedLightningRod {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 27442 {
            return Some(WaxedLightningRod {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 27444 {
            return Some(WaxedLightningRod {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 27459 {
            return Some(WaxedLightningRod {
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::Down,
            });
        }
        if state_id == 27447 {
            return Some(WaxedLightningRod {
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
