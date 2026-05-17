use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpruceFenceGate {
    pub in_wall: bool,
    pub open: bool,
    pub r#facing: Facing,
    pub powered: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for SpruceFenceGate {
    fn to_id(&self) -> i32 {
        if self.r#open == false && self.r#facing == Facing::South && self.r#powered == false && self.r#in_wall == false { return 13297; }
        if self.r#powered == true && self.r#in_wall == false && self.r#facing == Facing::East && self.r#open == false { return 13312; }
        if self.r#facing == Facing::East && self.r#open == true && self.r#powered == true && self.r#in_wall == true { return 13306; }
        if self.r#in_wall == false && self.r#facing == Facing::South && self.r#powered == false && self.r#open == true { return 13295; }
        if self.r#powered == true && self.r#in_wall == false && self.r#facing == Facing::North && self.r#open == false { return 13288; }
        if self.r#open == false && self.r#powered == true && self.r#facing == Facing::North && self.r#in_wall == true { return 13284; }
        if self.r#powered == true && self.r#facing == Facing::East && self.r#open == true && self.r#in_wall == false { return 13310; }
        if self.r#open == false && self.r#powered == false && self.r#in_wall == true && self.r#facing == Facing::West { return 13301; }
        if self.r#facing == Facing::South && self.r#in_wall == true && self.r#open == false && self.r#powered == false { return 13293; }
        if self.r#facing == Facing::South && self.r#open == false && self.r#in_wall == true && self.r#powered == true { return 13292; }
        if self.r#facing == Facing::West && self.r#powered == false && self.r#in_wall == false && self.r#open == false { return 13305; }
        if self.r#powered == true && self.r#facing == Facing::West && self.r#in_wall == true && self.r#open == false { return 13300; }
        if self.r#in_wall == false && self.r#open == false && self.r#facing == Facing::South && self.r#powered == true { return 13296; }
        if self.r#in_wall == false && self.r#open == true && self.r#facing == Facing::East && self.r#powered == false { return 13311; }
        if self.r#powered == false && self.r#in_wall == true && self.r#facing == Facing::East && self.r#open == false { return 13309; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#in_wall == true && self.r#open == true { return 13298; }
        if self.r#in_wall == false && self.r#powered == false && self.r#open == true && self.r#facing == Facing::West { return 13303; }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#open == true && self.r#in_wall == true { return 13290; }
        if self.r#open == true && self.r#facing == Facing::North && self.r#powered == false && self.r#in_wall == true { return 13283; }
        if self.r#open == false && self.r#powered == false && self.r#facing == Facing::North && self.r#in_wall == true { return 13285; }
        if self.r#in_wall == false && self.r#powered == false && self.r#facing == Facing::East && self.r#open == false { return 13313; }
        if self.r#in_wall == false && self.r#powered == true && self.r#open == true && self.r#facing == Facing::West { return 13302; }
        if self.r#in_wall == true && self.r#powered == false && self.r#facing == Facing::West && self.r#open == true { return 13299; }
        if self.r#facing == Facing::North && self.r#open == false && self.r#powered == false && self.r#in_wall == false { return 13289; }
        if self.r#in_wall == false && self.r#facing == Facing::North && self.r#powered == true && self.r#open == true { return 13286; }
        if self.r#open == true && self.r#in_wall == false && self.r#powered == false && self.r#facing == Facing::North { return 13287; }
        if self.r#powered == true && self.r#in_wall == false && self.r#facing == Facing::West && self.r#open == false { return 13304; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#in_wall == true && self.r#open == true { return 13291; }
        if self.r#in_wall == true && self.r#open == true && self.r#powered == false && self.r#facing == Facing::East { return 13307; }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#open == true && self.r#in_wall == true { return 13282; }
        if self.r#open == true && self.r#powered == true && self.r#in_wall == false && self.r#facing == Facing::South { return 13294; }
        if self.r#open == false && self.r#powered == true && self.r#facing == Facing::East && self.r#in_wall == true { return 13308; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13297 {
            return Some(SpruceFenceGate {
                r#open: false,
                r#facing: Facing::South,
                r#powered: false,
                r#in_wall: false,
            });
        }
        if state_id == 13312 {
            return Some(SpruceFenceGate {
                r#powered: true,
                r#in_wall: false,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 13306 {
            return Some(SpruceFenceGate {
                r#facing: Facing::East,
                r#open: true,
                r#powered: true,
                r#in_wall: true,
            });
        }
        if state_id == 13295 {
            return Some(SpruceFenceGate {
                r#in_wall: false,
                r#facing: Facing::South,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 13288 {
            return Some(SpruceFenceGate {
                r#powered: true,
                r#in_wall: false,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 13284 {
            return Some(SpruceFenceGate {
                r#open: false,
                r#powered: true,
                r#facing: Facing::North,
                r#in_wall: true,
            });
        }
        if state_id == 13310 {
            return Some(SpruceFenceGate {
                r#powered: true,
                r#facing: Facing::East,
                r#open: true,
                r#in_wall: false,
            });
        }
        if state_id == 13301 {
            return Some(SpruceFenceGate {
                r#open: false,
                r#powered: false,
                r#in_wall: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 13293 {
            return Some(SpruceFenceGate {
                r#facing: Facing::South,
                r#in_wall: true,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 13292 {
            return Some(SpruceFenceGate {
                r#facing: Facing::South,
                r#open: false,
                r#in_wall: true,
                r#powered: true,
            });
        }
        if state_id == 13305 {
            return Some(SpruceFenceGate {
                r#facing: Facing::West,
                r#powered: false,
                r#in_wall: false,
                r#open: false,
            });
        }
        if state_id == 13300 {
            return Some(SpruceFenceGate {
                r#powered: true,
                r#facing: Facing::West,
                r#in_wall: true,
                r#open: false,
            });
        }
        if state_id == 13296 {
            return Some(SpruceFenceGate {
                r#in_wall: false,
                r#open: false,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 13311 {
            return Some(SpruceFenceGate {
                r#in_wall: false,
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 13309 {
            return Some(SpruceFenceGate {
                r#powered: false,
                r#in_wall: true,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 13298 {
            return Some(SpruceFenceGate {
                r#facing: Facing::West,
                r#powered: true,
                r#in_wall: true,
                r#open: true,
            });
        }
        if state_id == 13303 {
            return Some(SpruceFenceGate {
                r#in_wall: false,
                r#powered: false,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 13290 {
            return Some(SpruceFenceGate {
                r#facing: Facing::South,
                r#powered: true,
                r#open: true,
                r#in_wall: true,
            });
        }
        if state_id == 13283 {
            return Some(SpruceFenceGate {
                r#open: true,
                r#facing: Facing::North,
                r#powered: false,
                r#in_wall: true,
            });
        }
        if state_id == 13285 {
            return Some(SpruceFenceGate {
                r#open: false,
                r#powered: false,
                r#facing: Facing::North,
                r#in_wall: true,
            });
        }
        if state_id == 13313 {
            return Some(SpruceFenceGate {
                r#in_wall: false,
                r#powered: false,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 13302 {
            return Some(SpruceFenceGate {
                r#in_wall: false,
                r#powered: true,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 13299 {
            return Some(SpruceFenceGate {
                r#in_wall: true,
                r#powered: false,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 13289 {
            return Some(SpruceFenceGate {
                r#facing: Facing::North,
                r#open: false,
                r#powered: false,
                r#in_wall: false,
            });
        }
        if state_id == 13286 {
            return Some(SpruceFenceGate {
                r#in_wall: false,
                r#facing: Facing::North,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 13287 {
            return Some(SpruceFenceGate {
                r#open: true,
                r#in_wall: false,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 13304 {
            return Some(SpruceFenceGate {
                r#powered: true,
                r#in_wall: false,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 13291 {
            return Some(SpruceFenceGate {
                r#powered: false,
                r#facing: Facing::South,
                r#in_wall: true,
                r#open: true,
            });
        }
        if state_id == 13307 {
            return Some(SpruceFenceGate {
                r#in_wall: true,
                r#open: true,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 13282 {
            return Some(SpruceFenceGate {
                r#facing: Facing::North,
                r#powered: true,
                r#open: true,
                r#in_wall: true,
            });
        }
        if state_id == 13294 {
            return Some(SpruceFenceGate {
                r#open: true,
                r#powered: true,
                r#in_wall: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 13308 {
            return Some(SpruceFenceGate {
                r#open: false,
                r#powered: true,
                r#facing: Facing::East,
                r#in_wall: true,
            });
        }
        return None;
    }
}

