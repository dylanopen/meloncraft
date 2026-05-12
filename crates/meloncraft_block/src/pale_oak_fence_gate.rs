use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleOakFenceGate {
    pub open: bool,
    pub powered: bool,
    pub in_wall: bool,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for PaleOakFenceGate {
    fn to_id(self) -> i32 {
        if block_state.r#in_wall == false && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#open == true { return 13494; }
        if block_state.r#in_wall == false && block_state.r#powered == true && block_state.r#open == false && block_state.r#facing == Facing::East { return 13504; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#in_wall == true { return 13474; }
        if block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == true && block_state.r#in_wall == true { return 13492; }
        if block_state.r#in_wall == true && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#open == true { return 13498; }
        if block_state.r#facing == Facing::South && block_state.r#in_wall == true && block_state.r#open == false && block_state.r#powered == true { return 13484; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#in_wall == true && block_state.r#open == true { return 13499; }
        if block_state.r#facing == Facing::South && block_state.r#in_wall == false && block_state.r#powered == true && block_state.r#open == false { return 13488; }
        if block_state.r#in_wall == false && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#open == false { return 13481; }
        if block_state.r#powered == false && block_state.r#in_wall == false && block_state.r#facing == Facing::South && block_state.r#open == false { return 13489; }
        if block_state.r#in_wall == true && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#powered == true { return 13482; }
        if block_state.r#in_wall == true && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#powered == false { return 13485; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#in_wall == true { return 13501; }
        if block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#in_wall == false && block_state.r#powered == false { return 13505; }
        if block_state.r#in_wall == false && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#open == true { return 13479; }
        if block_state.r#facing == Facing::North && block_state.r#in_wall == false && block_state.r#open == false && block_state.r#powered == true { return 13480; }
        if block_state.r#facing == Facing::West && block_state.r#in_wall == true && block_state.r#powered == false && block_state.r#open == false { return 13493; }
        if block_state.r#in_wall == false && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == false { return 13497; }
        if block_state.r#in_wall == false && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::South { return 13487; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#in_wall == true && block_state.r#open == false { return 13477; }
        if block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#in_wall == true && block_state.r#powered == true { return 13476; }
        if block_state.r#powered == true && block_state.r#in_wall == false && block_state.r#open == true && block_state.r#facing == Facing::North { return 13478; }
        if block_state.r#powered == true && block_state.r#open == false && block_state.r#in_wall == true && block_state.r#facing == Facing::East { return 13500; }
        if block_state.r#powered == true && block_state.r#in_wall == false && block_state.r#facing == Facing::East && block_state.r#open == true { return 13502; }
        if block_state.r#in_wall == true && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::North { return 13475; }
        if block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#in_wall == false && block_state.r#powered == false { return 13495; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#in_wall == false { return 13486; }
        if block_state.r#in_wall == true && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == false { return 13483; }
        if block_state.r#powered == true && block_state.r#in_wall == false && block_state.r#facing == Facing::West && block_state.r#open == false { return 13496; }
        if block_state.r#in_wall == false && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#open == true { return 13503; }
        if block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#in_wall == true { return 13491; }
        if block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#in_wall == true { return 13490; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13494 {
            return Some(PaleOakFenceGate {
                r#in_wall: false,
                r#powered: true,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 13504 {
            return Some(PaleOakFenceGate {
                r#in_wall: false,
                r#powered: true,
                r#open: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 13474 {
            return Some(PaleOakFenceGate {
                r#open: true,
                r#powered: true,
                r#facing: Facing::North,
                r#in_wall: true,
            });
        }
        if state_id == 13492 {
            return Some(PaleOakFenceGate {
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
                r#in_wall: true,
            });
        }
        if state_id == 13498 {
            return Some(PaleOakFenceGate {
                r#in_wall: true,
                r#powered: true,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 13484 {
            return Some(PaleOakFenceGate {
                r#facing: Facing::South,
                r#in_wall: true,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 13499 {
            return Some(PaleOakFenceGate {
                r#facing: Facing::East,
                r#powered: false,
                r#in_wall: true,
                r#open: true,
            });
        }
        if state_id == 13488 {
            return Some(PaleOakFenceGate {
                r#facing: Facing::South,
                r#in_wall: false,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 13481 {
            return Some(PaleOakFenceGate {
                r#in_wall: false,
                r#powered: false,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 13489 {
            return Some(PaleOakFenceGate {
                r#powered: false,
                r#in_wall: false,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 13482 {
            return Some(PaleOakFenceGate {
                r#in_wall: true,
                r#open: true,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 13485 {
            return Some(PaleOakFenceGate {
                r#in_wall: true,
                r#open: false,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 13501 {
            return Some(PaleOakFenceGate {
                r#powered: false,
                r#open: false,
                r#facing: Facing::East,
                r#in_wall: true,
            });
        }
        if state_id == 13505 {
            return Some(PaleOakFenceGate {
                r#open: false,
                r#facing: Facing::East,
                r#in_wall: false,
                r#powered: false,
            });
        }
        if state_id == 13479 {
            return Some(PaleOakFenceGate {
                r#in_wall: false,
                r#powered: false,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 13480 {
            return Some(PaleOakFenceGate {
                r#facing: Facing::North,
                r#in_wall: false,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 13493 {
            return Some(PaleOakFenceGate {
                r#facing: Facing::West,
                r#in_wall: true,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 13497 {
            return Some(PaleOakFenceGate {
                r#in_wall: false,
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 13487 {
            return Some(PaleOakFenceGate {
                r#in_wall: false,
                r#open: true,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 13477 {
            return Some(PaleOakFenceGate {
                r#facing: Facing::North,
                r#powered: false,
                r#in_wall: true,
                r#open: false,
            });
        }
        if state_id == 13476 {
            return Some(PaleOakFenceGate {
                r#facing: Facing::North,
                r#open: false,
                r#in_wall: true,
                r#powered: true,
            });
        }
        if state_id == 13478 {
            return Some(PaleOakFenceGate {
                r#powered: true,
                r#in_wall: false,
                r#open: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 13500 {
            return Some(PaleOakFenceGate {
                r#powered: true,
                r#open: false,
                r#in_wall: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 13502 {
            return Some(PaleOakFenceGate {
                r#powered: true,
                r#in_wall: false,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 13475 {
            return Some(PaleOakFenceGate {
                r#in_wall: true,
                r#open: true,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 13495 {
            return Some(PaleOakFenceGate {
                r#open: true,
                r#facing: Facing::West,
                r#in_wall: false,
                r#powered: false,
            });
        }
        if state_id == 13486 {
            return Some(PaleOakFenceGate {
                r#open: true,
                r#powered: true,
                r#facing: Facing::South,
                r#in_wall: false,
            });
        }
        if state_id == 13483 {
            return Some(PaleOakFenceGate {
                r#in_wall: true,
                r#facing: Facing::South,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 13496 {
            return Some(PaleOakFenceGate {
                r#powered: true,
                r#in_wall: false,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 13503 {
            return Some(PaleOakFenceGate {
                r#in_wall: false,
                r#facing: Facing::East,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 13491 {
            return Some(PaleOakFenceGate {
                r#open: true,
                r#facing: Facing::West,
                r#powered: false,
                r#in_wall: true,
            });
        }
        if state_id == 13490 {
            return Some(PaleOakFenceGate {
                r#open: true,
                r#facing: Facing::West,
                r#powered: true,
                r#in_wall: true,
            });
        }
        return None;
    }
}

