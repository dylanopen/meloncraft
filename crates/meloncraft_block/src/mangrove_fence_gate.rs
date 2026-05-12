use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangroveFenceGate {
    pub in_wall: bool,
    pub r#facing: Facing,
    pub powered: bool,
    pub open: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for MangroveFenceGate {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#in_wall == true && block_state.r#open == false { return 13509; }
        if block_state.r#in_wall == true && block_state.r#powered == true && block_state.r#open == false && block_state.r#facing == Facing::East { return 13532; }
        if block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#in_wall == true && block_state.r#powered == true { return 13508; }
        if block_state.r#powered == true && block_state.r#open == true && block_state.r#in_wall == true && block_state.r#facing == Facing::West { return 13522; }
        if block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#in_wall == true && block_state.r#powered == false { return 13525; }
        if block_state.r#open == true && block_state.r#in_wall == false && block_state.r#powered == false && block_state.r#facing == Facing::West { return 13527; }
        if block_state.r#powered == true && block_state.r#in_wall == false && block_state.r#open == false && block_state.r#facing == Facing::North { return 13512; }
        if block_state.r#in_wall == true && block_state.r#powered == true && block_state.r#open == false && block_state.r#facing == Facing::South { return 13516; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#in_wall == true && block_state.r#facing == Facing::West { return 13524; }
        if block_state.r#in_wall == false && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::West { return 13528; }
        if block_state.r#in_wall == true && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::East { return 13531; }
        if block_state.r#in_wall == false && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::South { return 13519; }
        if block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#in_wall == true && block_state.r#powered == false { return 13533; }
        if block_state.r#in_wall == true && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#open == true { return 13523; }
        if block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#powered == false && block_state.r#in_wall == false { return 13511; }
        if block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == true && block_state.r#in_wall == false { return 13518; }
        if block_state.r#in_wall == true && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == true { return 13506; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#in_wall == false && block_state.r#open == false { return 13537; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#powered == true && block_state.r#in_wall == false { return 13534; }
        if block_state.r#open == true && block_state.r#in_wall == false && block_state.r#facing == Facing::West && block_state.r#powered == true { return 13526; }
        if block_state.r#open == true && block_state.r#in_wall == false && block_state.r#facing == Facing::East && block_state.r#powered == false { return 13535; }
        if block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == false && block_state.r#in_wall == false { return 13513; }
        if block_state.r#facing == Facing::East && block_state.r#in_wall == true && block_state.r#open == true && block_state.r#powered == true { return 13530; }
        if block_state.r#in_wall == false && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#open == true { return 13510; }
        if block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#in_wall == false && block_state.r#powered == true { return 13536; }
        if block_state.r#in_wall == true && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == true { return 13514; }
        if block_state.r#in_wall == true && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#powered == false { return 13517; }
        if block_state.r#in_wall == false && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::South { return 13520; }
        if block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#in_wall == true { return 13507; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#in_wall == false && block_state.r#facing == Facing::South { return 13521; }
        if block_state.r#in_wall == false && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#powered == false { return 13529; }
        if block_state.r#in_wall == true && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::South { return 13515; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13509 {
            return Some(MangroveFenceGate {
                r#facing: Facing::North,
                r#powered: false,
                r#in_wall: true,
                r#open: false,
            });
        }
        if state_id == 13532 {
            return Some(MangroveFenceGate {
                r#in_wall: true,
                r#powered: true,
                r#open: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 13508 {
            return Some(MangroveFenceGate {
                r#open: false,
                r#facing: Facing::North,
                r#in_wall: true,
                r#powered: true,
            });
        }
        if state_id == 13522 {
            return Some(MangroveFenceGate {
                r#powered: true,
                r#open: true,
                r#in_wall: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 13525 {
            return Some(MangroveFenceGate {
                r#open: false,
                r#facing: Facing::West,
                r#in_wall: true,
                r#powered: false,
            });
        }
        if state_id == 13527 {
            return Some(MangroveFenceGate {
                r#open: true,
                r#in_wall: false,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 13512 {
            return Some(MangroveFenceGate {
                r#powered: true,
                r#in_wall: false,
                r#open: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 13516 {
            return Some(MangroveFenceGate {
                r#in_wall: true,
                r#powered: true,
                r#open: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 13524 {
            return Some(MangroveFenceGate {
                r#open: false,
                r#powered: true,
                r#in_wall: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 13528 {
            return Some(MangroveFenceGate {
                r#in_wall: false,
                r#open: false,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 13531 {
            return Some(MangroveFenceGate {
                r#in_wall: true,
                r#open: true,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 13519 {
            return Some(MangroveFenceGate {
                r#in_wall: false,
                r#open: true,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 13533 {
            return Some(MangroveFenceGate {
                r#open: false,
                r#facing: Facing::East,
                r#in_wall: true,
                r#powered: false,
            });
        }
        if state_id == 13523 {
            return Some(MangroveFenceGate {
                r#in_wall: true,
                r#powered: false,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 13511 {
            return Some(MangroveFenceGate {
                r#facing: Facing::North,
                r#open: true,
                r#powered: false,
                r#in_wall: false,
            });
        }
        if state_id == 13518 {
            return Some(MangroveFenceGate {
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
                r#in_wall: false,
            });
        }
        if state_id == 13506 {
            return Some(MangroveFenceGate {
                r#in_wall: true,
                r#open: true,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 13537 {
            return Some(MangroveFenceGate {
                r#facing: Facing::East,
                r#powered: false,
                r#in_wall: false,
                r#open: false,
            });
        }
        if state_id == 13534 {
            return Some(MangroveFenceGate {
                r#facing: Facing::East,
                r#open: true,
                r#powered: true,
                r#in_wall: false,
            });
        }
        if state_id == 13526 {
            return Some(MangroveFenceGate {
                r#open: true,
                r#in_wall: false,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 13535 {
            return Some(MangroveFenceGate {
                r#open: true,
                r#in_wall: false,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 13513 {
            return Some(MangroveFenceGate {
                r#facing: Facing::North,
                r#open: false,
                r#powered: false,
                r#in_wall: false,
            });
        }
        if state_id == 13530 {
            return Some(MangroveFenceGate {
                r#facing: Facing::East,
                r#in_wall: true,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 13510 {
            return Some(MangroveFenceGate {
                r#in_wall: false,
                r#powered: true,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 13536 {
            return Some(MangroveFenceGate {
                r#open: false,
                r#facing: Facing::East,
                r#in_wall: false,
                r#powered: true,
            });
        }
        if state_id == 13514 {
            return Some(MangroveFenceGate {
                r#in_wall: true,
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 13517 {
            return Some(MangroveFenceGate {
                r#in_wall: true,
                r#open: false,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 13520 {
            return Some(MangroveFenceGate {
                r#in_wall: false,
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 13507 {
            return Some(MangroveFenceGate {
                r#open: true,
                r#facing: Facing::North,
                r#powered: false,
                r#in_wall: true,
            });
        }
        if state_id == 13521 {
            return Some(MangroveFenceGate {
                r#powered: false,
                r#open: false,
                r#in_wall: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 13529 {
            return Some(MangroveFenceGate {
                r#in_wall: false,
                r#open: false,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 13515 {
            return Some(MangroveFenceGate {
                r#in_wall: true,
                r#open: true,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

