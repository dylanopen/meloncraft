use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposedLightningRod {
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

impl BlockState for ExposedLightningRod {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#powered == true && self.r#facing == Facing::East {
            return 27370;
        }
        if self.r#powered == false && self.r#facing == Facing::Down && self.r#waterlogged == true {
            return 27387;
        }
        if self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::Up {
            return 27382;
        }
        if self.r#powered == true && self.r#facing == Facing::Down && self.r#waterlogged == true {
            return 27385;
        }
        if self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::North {
            return 27366;
        }
        if self.r#facing == Facing::Down && self.r#powered == false && self.r#waterlogged == false {
            return 27388;
        }
        if self.r#waterlogged == false && self.r#powered == false && self.r#facing == Facing::North
        {
            return 27368;
        }
        if self.r#powered == true && self.r#waterlogged == true && self.r#facing == Facing::East {
            return 27369;
        }
        if self.r#powered == false && self.r#waterlogged == true && self.r#facing == Facing::West {
            return 27379;
        }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#waterlogged == true {
            return 27371;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#powered == false {
            return 27375;
        }
        if self.r#facing == Facing::Up && self.r#powered == true && self.r#waterlogged == true {
            return 27381;
        }
        if self.r#facing == Facing::North && self.r#powered == false && self.r#waterlogged == true {
            return 27367;
        }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#waterlogged == false {
            return 27374;
        }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#powered == true {
            return 27365;
        }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#powered == false
        {
            return 27376;
        }
        if self.r#powered == false && self.r#facing == Facing::Up && self.r#waterlogged == true {
            return 27383;
        }
        if self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::East {
            return 27372;
        }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#waterlogged == true {
            return 27377;
        }
        if self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::West {
            return 27378;
        }
        if self.r#powered == false && self.r#facing == Facing::West && self.r#waterlogged == false {
            return 27380;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::Up && self.r#powered == false {
            return 27384;
        }
        if self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::Down {
            return 27386;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#powered == true {
            return 27373;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27370 {
            return Some(ExposedLightningRod {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27387 {
            return Some(ExposedLightningRod {
                r#powered: false,
                r#facing: Facing::Down,
                r#waterlogged: true,
            });
        }
        if state_id == 27382 {
            return Some(ExposedLightningRod {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::Up,
            });
        }
        if state_id == 27385 {
            return Some(ExposedLightningRod {
                r#powered: true,
                r#facing: Facing::Down,
                r#waterlogged: true,
            });
        }
        if state_id == 27366 {
            return Some(ExposedLightningRod {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 27388 {
            return Some(ExposedLightningRod {
                r#facing: Facing::Down,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 27368 {
            return Some(ExposedLightningRod {
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 27369 {
            return Some(ExposedLightningRod {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27379 {
            return Some(ExposedLightningRod {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 27371 {
            return Some(ExposedLightningRod {
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27375 {
            return Some(ExposedLightningRod {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 27381 {
            return Some(ExposedLightningRod {
                r#facing: Facing::Up,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27367 {
            return Some(ExposedLightningRod {
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27374 {
            return Some(ExposedLightningRod {
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 27365 {
            return Some(ExposedLightningRod {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 27376 {
            return Some(ExposedLightningRod {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 27383 {
            return Some(ExposedLightningRod {
                r#powered: false,
                r#facing: Facing::Up,
                r#waterlogged: true,
            });
        }
        if state_id == 27372 {
            return Some(ExposedLightningRod {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 27377 {
            return Some(ExposedLightningRod {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27378 {
            return Some(ExposedLightningRod {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 27380 {
            return Some(ExposedLightningRod {
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 27384 {
            return Some(ExposedLightningRod {
                r#waterlogged: false,
                r#facing: Facing::Up,
                r#powered: false,
            });
        }
        if state_id == 27386 {
            return Some(ExposedLightningRod {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::Down,
            });
        }
        if state_id == 27373 {
            return Some(ExposedLightningRod {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        return None;
    }
}
