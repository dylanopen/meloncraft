use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightningRod {
    pub powered: bool,
    pub r#facing: Facing,
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

impl BlockState for LightningRod {
    fn to_id(&self) -> i32 {
        if self.r#powered == false && self.r#waterlogged == true && self.r#facing == Facing::South {
            return 27351;
        }
        if self.r#facing == Facing::Down && self.r#powered == false && self.r#waterlogged == true {
            return 27363;
        }
        if self.r#facing == Facing::North && self.r#powered == false && self.r#waterlogged == false
        {
            return 27344;
        }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#powered == true {
            return 27342;
        }
        if self.r#powered == true && self.r#facing == Facing::East && self.r#waterlogged == true {
            return 27345;
        }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#waterlogged == true {
            return 27349;
        }
        if self.r#facing == Facing::West && self.r#powered == false && self.r#waterlogged == true {
            return 27355;
        }
        if self.r#waterlogged == true && self.r#powered == false && self.r#facing == Facing::North {
            return 27343;
        }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#waterlogged == false {
            return 27348;
        }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#waterlogged == true {
            return 27353;
        }
        if self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::South
        {
            return 27352;
        }
        if self.r#facing == Facing::Up && self.r#waterlogged == true && self.r#powered == true {
            return 27357;
        }
        if self.r#facing == Facing::Up && self.r#powered == false && self.r#waterlogged == true {
            return 27359;
        }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#powered == true {
            return 27346;
        }
        if self.r#facing == Facing::West && self.r#powered == false && self.r#waterlogged == false {
            return 27356;
        }
        if self.r#waterlogged == false && self.r#powered == true && self.r#facing == Facing::West {
            return 27354;
        }
        if self.r#waterlogged == false && self.r#powered == true && self.r#facing == Facing::Up {
            return 27358;
        }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#powered == false {
            return 27347;
        }
        if self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::South {
            return 27350;
        }
        if self.r#waterlogged == true && self.r#facing == Facing::Down && self.r#powered == true {
            return 27361;
        }
        if self.r#powered == false && self.r#facing == Facing::Up && self.r#waterlogged == false {
            return 27360;
        }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#powered == true {
            return 27341;
        }
        if self.r#powered == false && self.r#facing == Facing::Down && self.r#waterlogged == false {
            return 27364;
        }
        if self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::Down {
            return 27362;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27351 {
            return Some(LightningRod {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 27363 {
            return Some(LightningRod {
                r#facing: Facing::Down,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27344 {
            return Some(LightningRod {
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 27342 {
            return Some(LightningRod {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 27345 {
            return Some(LightningRod {
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 27349 {
            return Some(LightningRod {
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27355 {
            return Some(LightningRod {
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27343 {
            return Some(LightningRod {
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 27348 {
            return Some(LightningRod {
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 27353 {
            return Some(LightningRod {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27352 {
            return Some(LightningRod {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 27357 {
            return Some(LightningRod {
                r#facing: Facing::Up,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 27359 {
            return Some(LightningRod {
                r#facing: Facing::Up,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27346 {
            return Some(LightningRod {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 27356 {
            return Some(LightningRod {
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 27354 {
            return Some(LightningRod {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 27358 {
            return Some(LightningRod {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::Up,
            });
        }
        if state_id == 27347 {
            return Some(LightningRod {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 27350 {
            return Some(LightningRod {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 27361 {
            return Some(LightningRod {
                r#waterlogged: true,
                r#facing: Facing::Down,
                r#powered: true,
            });
        }
        if state_id == 27360 {
            return Some(LightningRod {
                r#powered: false,
                r#facing: Facing::Up,
                r#waterlogged: false,
            });
        }
        if state_id == 27341 {
            return Some(LightningRod {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 27364 {
            return Some(LightningRod {
                r#powered: false,
                r#facing: Facing::Down,
                r#waterlogged: false,
            });
        }
        if state_id == 27362 {
            return Some(LightningRod {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::Down,
            });
        }
        return None;
    }
}
