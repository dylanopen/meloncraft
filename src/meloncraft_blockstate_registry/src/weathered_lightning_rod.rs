use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatheredLightningRod {
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

impl BlockState for WeatheredLightningRod {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West && self.r#powered == false && self.r#waterlogged == false { return 27404; }
        if self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::Down { return 27410; }
        if self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::North { return 27392; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#powered == true { return 27402; }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#waterlogged == true { return 27395; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#powered == true { return 27390; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#powered == false { return 27403; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#waterlogged == true { return 27399; }
        if self.r#powered == true && self.r#waterlogged == true && self.r#facing == Facing::East { return 27393; }
        if self.r#facing == Facing::Up && self.r#powered == false && self.r#waterlogged == true { return 27407; }
        if self.r#powered == true && self.r#facing == Facing::East && self.r#waterlogged == false { return 27394; }
        if self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::South { return 27400; }
        if self.r#facing == Facing::Up && self.r#waterlogged == true && self.r#powered == true { return 27405; }
        if self.r#powered == true && self.r#waterlogged == true && self.r#facing == Facing::North { return 27389; }
        if self.r#powered == true && self.r#facing == Facing::Up && self.r#waterlogged == false { return 27406; }
        if self.r#facing == Facing::Down && self.r#waterlogged == false && self.r#powered == false { return 27412; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#powered == true { return 27398; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#powered == false { return 27396; }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#waterlogged == true { return 27397; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#powered == true { return 27401; }
        if self.r#powered == false && self.r#facing == Facing::North && self.r#waterlogged == true { return 27391; }
        if self.r#facing == Facing::Up && self.r#powered == false && self.r#waterlogged == false { return 27408; }
        if self.r#waterlogged == true && self.r#facing == Facing::Down && self.r#powered == true { return 27409; }
        if self.r#facing == Facing::Down && self.r#waterlogged == true && self.r#powered == false { return 27411; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27404 {
            return Some(WeatheredLightningRod {
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 27410 {
            return Some(WeatheredLightningRod {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::Down,
            });
        }
        if state_id == 27392 {
            return Some(WeatheredLightningRod {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 27402 {
            return Some(WeatheredLightningRod {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 27395 {
            return Some(WeatheredLightningRod {
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27390 {
            return Some(WeatheredLightningRod {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 27403 {
            return Some(WeatheredLightningRod {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 27399 {
            return Some(WeatheredLightningRod {
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 27393 {
            return Some(WeatheredLightningRod {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 27407 {
            return Some(WeatheredLightningRod {
                r#facing: Facing::Up,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 27394 {
            return Some(WeatheredLightningRod {
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 27400 {
            return Some(WeatheredLightningRod {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 27405 {
            return Some(WeatheredLightningRod {
                r#facing: Facing::Up,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 27389 {
            return Some(WeatheredLightningRod {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 27406 {
            return Some(WeatheredLightningRod {
                r#powered: true,
                r#facing: Facing::Up,
                r#waterlogged: false,
            });
        }
        if state_id == 27412 {
            return Some(WeatheredLightningRod {
                r#facing: Facing::Down,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 27398 {
            return Some(WeatheredLightningRod {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 27396 {
            return Some(WeatheredLightningRod {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 27397 {
            return Some(WeatheredLightningRod {
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 27401 {
            return Some(WeatheredLightningRod {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 27391 {
            return Some(WeatheredLightningRod {
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 27408 {
            return Some(WeatheredLightningRod {
                r#facing: Facing::Up,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 27409 {
            return Some(WeatheredLightningRod {
                r#waterlogged: true,
                r#facing: Facing::Down,
                r#powered: true,
            });
        }
        if state_id == 27411 {
            return Some(WeatheredLightningRod {
                r#facing: Facing::Down,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        return None;
    }
}

