use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakButton {
    pub r#face: Face,
    pub r#facing: Facing,
    pub powered: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Face {
    Floor,
    Wall,
    Ceiling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for OakButton {
    fn to_id(self) -> i32 {
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::North && block_state.r#powered == false { return 10474; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::South && block_state.r#powered == false { return 10492; }
        if block_state.r#powered == true && block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::North { return 10489; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#face == Face::Ceiling { return 10493; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#face == Face::Floor { return 10478; }
        if block_state.r#powered == true && block_state.r#face == Face::Wall && block_state.r#facing == Facing::North { return 10481; }
        if block_state.r#powered == false && block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::East { return 10496; }
        if block_state.r#face == Face::Floor && block_state.r#powered == true && block_state.r#facing == Facing::South { return 10475; }
        if block_state.r#facing == Facing::East && block_state.r#face == Face::Wall && block_state.r#powered == true { return 10487; }
        if block_state.r#facing == Facing::West && block_state.r#face == Face::Wall && block_state.r#powered == false { return 10486; }
        if block_state.r#face == Face::Wall && block_state.r#powered == false && block_state.r#facing == Facing::North { return 10482; }
        if block_state.r#face == Face::Floor && block_state.r#powered == true && block_state.r#facing == Facing::East { return 10479; }
        if block_state.r#face == Face::Floor && block_state.r#powered == true && block_state.r#facing == Facing::West { return 10477; }
        if block_state.r#facing == Facing::South && block_state.r#face == Face::Wall && block_state.r#powered == true { return 10483; }
        if block_state.r#powered == false && block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::West { return 10494; }
        if block_state.r#face == Face::Ceiling && block_state.r#facing == Facing::North && block_state.r#powered == false { return 10490; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#face == Face::Wall { return 10488; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#face == Face::Ceiling { return 10495; }
        if block_state.r#face == Face::Wall && block_state.r#facing == Facing::West && block_state.r#powered == true { return 10485; }
        if block_state.r#face == Face::Floor && block_state.r#powered == true && block_state.r#facing == Facing::North { return 10473; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#face == Face::Ceiling { return 10491; }
        if block_state.r#powered == false && block_state.r#face == Face::Wall && block_state.r#facing == Facing::South { return 10484; }
        if block_state.r#face == Face::Floor && block_state.r#facing == Facing::South && block_state.r#powered == false { return 10476; }
        if block_state.r#powered == false && block_state.r#face == Face::Floor && block_state.r#facing == Facing::East { return 10480; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10474 {
            return Some(OakButton {
                r#face: Face::Floor,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 10492 {
            return Some(OakButton {
                r#face: Face::Ceiling,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 10489 {
            return Some(OakButton {
                r#powered: true,
                r#face: Face::Ceiling,
                r#facing: Facing::North,
            });
        }
        if state_id == 10493 {
            return Some(OakButton {
                r#powered: true,
                r#facing: Facing::West,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10478 {
            return Some(OakButton {
                r#facing: Facing::West,
                r#powered: false,
                r#face: Face::Floor,
            });
        }
        if state_id == 10481 {
            return Some(OakButton {
                r#powered: true,
                r#face: Face::Wall,
                r#facing: Facing::North,
            });
        }
        if state_id == 10496 {
            return Some(OakButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::East,
            });
        }
        if state_id == 10475 {
            return Some(OakButton {
                r#face: Face::Floor,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 10487 {
            return Some(OakButton {
                r#facing: Facing::East,
                r#face: Face::Wall,
                r#powered: true,
            });
        }
        if state_id == 10486 {
            return Some(OakButton {
                r#facing: Facing::West,
                r#face: Face::Wall,
                r#powered: false,
            });
        }
        if state_id == 10482 {
            return Some(OakButton {
                r#face: Face::Wall,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 10479 {
            return Some(OakButton {
                r#face: Face::Floor,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 10477 {
            return Some(OakButton {
                r#face: Face::Floor,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 10483 {
            return Some(OakButton {
                r#facing: Facing::South,
                r#face: Face::Wall,
                r#powered: true,
            });
        }
        if state_id == 10494 {
            return Some(OakButton {
                r#powered: false,
                r#face: Face::Ceiling,
                r#facing: Facing::West,
            });
        }
        if state_id == 10490 {
            return Some(OakButton {
                r#face: Face::Ceiling,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 10488 {
            return Some(OakButton {
                r#facing: Facing::East,
                r#powered: false,
                r#face: Face::Wall,
            });
        }
        if state_id == 10495 {
            return Some(OakButton {
                r#powered: true,
                r#facing: Facing::East,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10485 {
            return Some(OakButton {
                r#face: Face::Wall,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 10473 {
            return Some(OakButton {
                r#face: Face::Floor,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 10491 {
            return Some(OakButton {
                r#facing: Facing::South,
                r#powered: true,
                r#face: Face::Ceiling,
            });
        }
        if state_id == 10484 {
            return Some(OakButton {
                r#powered: false,
                r#face: Face::Wall,
                r#facing: Facing::South,
            });
        }
        if state_id == 10476 {
            return Some(OakButton {
                r#face: Face::Floor,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 10480 {
            return Some(OakButton {
                r#powered: false,
                r#face: Face::Floor,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

