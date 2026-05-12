use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchFenceGate {
    pub powered: bool,
    pub r#facing: Facing,
    pub open: bool,
    pub in_wall: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for BirchFenceGate {
    fn to_id(self) -> i32 {
        if block_state.r#in_wall == true && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == true { return 13332; }
        if block_state.r#in_wall == false && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == false { return 13321; }
        if block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#in_wall == false && block_state.r#powered == false { return 13345; }
        if block_state.r#in_wall == true && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#open == false { return 13324; }
        if block_state.r#open == false && block_state.r#in_wall == true && block_state.r#facing == Facing::East && block_state.r#powered == false { return 13341; }
        if block_state.r#in_wall == false && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#open == true { return 13342; }
        if block_state.r#facing == Facing::East && block_state.r#in_wall == true && block_state.r#open == true && block_state.r#powered == false { return 13339; }
        if block_state.r#facing == Facing::East && block_state.r#in_wall == true && block_state.r#open == false && block_state.r#powered == true { return 13340; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#in_wall == false { return 13337; }
        if block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#in_wall == false && block_state.r#powered == true { return 13336; }
        if block_state.r#open == true && block_state.r#in_wall == false && block_state.r#powered == true && block_state.r#facing == Facing::South { return 13326; }
        if block_state.r#powered == true && block_state.r#in_wall == false && block_state.r#open == false && block_state.r#facing == Facing::South { return 13328; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#in_wall == true && block_state.r#facing == Facing::North { return 13316; }
        if block_state.r#open == true && block_state.r#in_wall == true && block_state.r#facing == Facing::North && block_state.r#powered == true { return 13314; }
        if block_state.r#in_wall == false && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == false { return 13319; }
        if block_state.r#facing == Facing::South && block_state.r#in_wall == true && block_state.r#open == true && block_state.r#powered == false { return 13323; }
        if block_state.r#in_wall == true && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == true { return 13338; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#in_wall == false && block_state.r#powered == false { return 13343; }
        if block_state.r#powered == true && block_state.r#in_wall == true && block_state.r#facing == Facing::West && block_state.r#open == true { return 13330; }
        if block_state.r#in_wall == false && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::South { return 13327; }
        if block_state.r#facing == Facing::North && block_state.r#in_wall == true && block_state.r#open == true && block_state.r#powered == false { return 13315; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#in_wall == true { return 13325; }
        if block_state.r#in_wall == false && block_state.r#powered == true && block_state.r#open == false && block_state.r#facing == Facing::North { return 13320; }
        if block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#in_wall == true && block_state.r#powered == false { return 13333; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#open == true && block_state.r#in_wall == false { return 13334; }
        if block_state.r#powered == true && block_state.r#in_wall == false && block_state.r#facing == Facing::East && block_state.r#open == false { return 13344; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#in_wall == true && block_state.r#open == true { return 13331; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#in_wall == false && block_state.r#open == true { return 13335; }
        if block_state.r#facing == Facing::South && block_state.r#in_wall == true && block_state.r#open == true && block_state.r#powered == true { return 13322; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#open == false && block_state.r#in_wall == true { return 13317; }
        if block_state.r#powered == false && block_state.r#in_wall == false && block_state.r#facing == Facing::South && block_state.r#open == false { return 13329; }
        if block_state.r#powered == true && block_state.r#open == true && block_state.r#in_wall == false && block_state.r#facing == Facing::North { return 13318; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13332 {
            return Some(BirchFenceGate {
                r#in_wall: true,
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 13321 {
            return Some(BirchFenceGate {
                r#in_wall: false,
                r#facing: Facing::North,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 13345 {
            return Some(BirchFenceGate {
                r#open: false,
                r#facing: Facing::East,
                r#in_wall: false,
                r#powered: false,
            });
        }
        if state_id == 13324 {
            return Some(BirchFenceGate {
                r#in_wall: true,
                r#powered: true,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 13341 {
            return Some(BirchFenceGate {
                r#open: false,
                r#in_wall: true,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 13342 {
            return Some(BirchFenceGate {
                r#in_wall: false,
                r#facing: Facing::East,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 13339 {
            return Some(BirchFenceGate {
                r#facing: Facing::East,
                r#in_wall: true,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 13340 {
            return Some(BirchFenceGate {
                r#facing: Facing::East,
                r#in_wall: true,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 13337 {
            return Some(BirchFenceGate {
                r#open: false,
                r#powered: false,
                r#facing: Facing::West,
                r#in_wall: false,
            });
        }
        if state_id == 13336 {
            return Some(BirchFenceGate {
                r#open: false,
                r#facing: Facing::West,
                r#in_wall: false,
                r#powered: true,
            });
        }
        if state_id == 13326 {
            return Some(BirchFenceGate {
                r#open: true,
                r#in_wall: false,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 13328 {
            return Some(BirchFenceGate {
                r#powered: true,
                r#in_wall: false,
                r#open: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 13316 {
            return Some(BirchFenceGate {
                r#open: false,
                r#powered: true,
                r#in_wall: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 13314 {
            return Some(BirchFenceGate {
                r#open: true,
                r#in_wall: true,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 13319 {
            return Some(BirchFenceGate {
                r#in_wall: false,
                r#open: true,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 13323 {
            return Some(BirchFenceGate {
                r#facing: Facing::South,
                r#in_wall: true,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 13338 {
            return Some(BirchFenceGate {
                r#in_wall: true,
                r#open: true,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 13343 {
            return Some(BirchFenceGate {
                r#facing: Facing::East,
                r#open: true,
                r#in_wall: false,
                r#powered: false,
            });
        }
        if state_id == 13330 {
            return Some(BirchFenceGate {
                r#powered: true,
                r#in_wall: true,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 13327 {
            return Some(BirchFenceGate {
                r#in_wall: false,
                r#open: true,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 13315 {
            return Some(BirchFenceGate {
                r#facing: Facing::North,
                r#in_wall: true,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 13325 {
            return Some(BirchFenceGate {
                r#powered: false,
                r#facing: Facing::South,
                r#open: false,
                r#in_wall: true,
            });
        }
        if state_id == 13320 {
            return Some(BirchFenceGate {
                r#in_wall: false,
                r#powered: true,
                r#open: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 13333 {
            return Some(BirchFenceGate {
                r#open: false,
                r#facing: Facing::West,
                r#in_wall: true,
                r#powered: false,
            });
        }
        if state_id == 13334 {
            return Some(BirchFenceGate {
                r#facing: Facing::West,
                r#powered: true,
                r#open: true,
                r#in_wall: false,
            });
        }
        if state_id == 13344 {
            return Some(BirchFenceGate {
                r#powered: true,
                r#in_wall: false,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 13331 {
            return Some(BirchFenceGate {
                r#powered: false,
                r#facing: Facing::West,
                r#in_wall: true,
                r#open: true,
            });
        }
        if state_id == 13335 {
            return Some(BirchFenceGate {
                r#facing: Facing::West,
                r#powered: false,
                r#in_wall: false,
                r#open: true,
            });
        }
        if state_id == 13322 {
            return Some(BirchFenceGate {
                r#facing: Facing::South,
                r#in_wall: true,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 13317 {
            return Some(BirchFenceGate {
                r#facing: Facing::North,
                r#powered: false,
                r#open: false,
                r#in_wall: true,
            });
        }
        if state_id == 13329 {
            return Some(BirchFenceGate {
                r#powered: false,
                r#in_wall: false,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 13318 {
            return Some(BirchFenceGate {
                r#powered: true,
                r#open: true,
                r#in_wall: false,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

