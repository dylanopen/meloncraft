use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkOakFenceGate {
    pub r#facing: Facing,
    pub in_wall: bool,
    pub open: bool,
    pub powered: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for DarkOakFenceGate {
    fn to_id(self) -> i32 {
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#in_wall == false && block_state.r#facing == Facing::North { return 13449; }
        if block_state.r#in_wall == false && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#powered == true { return 13454; }
        if block_state.r#in_wall == true && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#open == true { return 13459; }
        if block_state.r#in_wall == true && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == false { return 13443; }
        if block_state.r#facing == Facing::East && block_state.r#in_wall == false && block_state.r#powered == false && block_state.r#open == true { return 13471; }
        if block_state.r#open == true && block_state.r#in_wall == true && block_state.r#powered == true && block_state.r#facing == Facing::North { return 13442; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#in_wall == false { return 13464; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#powered == true && block_state.r#in_wall == false { return 13470; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#open == true && block_state.r#in_wall == true { return 13450; }
        if block_state.r#open == true && block_state.r#in_wall == true && block_state.r#facing == Facing::East && block_state.r#powered == false { return 13467; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#in_wall == true && block_state.r#facing == Facing::East { return 13469; }
        if block_state.r#in_wall == false && block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::West { return 13463; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#open == false && block_state.r#in_wall == false { return 13456; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#in_wall == false && block_state.r#open == true { return 13446; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#open == false && block_state.r#in_wall == false { return 13448; }
        if block_state.r#in_wall == false && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == false { return 13455; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#open == false && block_state.r#in_wall == false { return 13472; }
        if block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#in_wall == false { return 13465; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#open == true && block_state.r#in_wall == true { return 13458; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#in_wall == true { return 13445; }
        if block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#in_wall == false && block_state.r#powered == true { return 13462; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#in_wall == false { return 13457; }
        if block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#in_wall == true && block_state.r#powered == false { return 13453; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#in_wall == true && block_state.r#powered == true { return 13466; }
        if block_state.r#facing == Facing::South && block_state.r#in_wall == true && block_state.r#powered == true && block_state.r#open == false { return 13452; }
        if block_state.r#facing == Facing::North && block_state.r#in_wall == false && block_state.r#open == true && block_state.r#powered == false { return 13447; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#in_wall == true { return 13461; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#open == true && block_state.r#in_wall == true { return 13451; }
        if block_state.r#in_wall == false && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::East { return 13473; }
        if block_state.r#facing == Facing::East && block_state.r#in_wall == true && block_state.r#open == false && block_state.r#powered == true { return 13468; }
        if block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == true && block_state.r#in_wall == true { return 13460; }
        if block_state.r#in_wall == true && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == true { return 13444; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13449 {
            return Some(DarkOakFenceGate {
                r#powered: false,
                r#open: false,
                r#in_wall: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 13454 {
            return Some(DarkOakFenceGate {
                r#in_wall: false,
                r#open: true,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 13459 {
            return Some(DarkOakFenceGate {
                r#in_wall: true,
                r#powered: false,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 13443 {
            return Some(DarkOakFenceGate {
                r#in_wall: true,
                r#open: true,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 13471 {
            return Some(DarkOakFenceGate {
                r#facing: Facing::East,
                r#in_wall: false,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 13442 {
            return Some(DarkOakFenceGate {
                r#open: true,
                r#in_wall: true,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 13464 {
            return Some(DarkOakFenceGate {
                r#powered: true,
                r#facing: Facing::West,
                r#open: false,
                r#in_wall: false,
            });
        }
        if state_id == 13470 {
            return Some(DarkOakFenceGate {
                r#facing: Facing::East,
                r#open: true,
                r#powered: true,
                r#in_wall: false,
            });
        }
        if state_id == 13450 {
            return Some(DarkOakFenceGate {
                r#facing: Facing::South,
                r#powered: true,
                r#open: true,
                r#in_wall: true,
            });
        }
        if state_id == 13467 {
            return Some(DarkOakFenceGate {
                r#open: true,
                r#in_wall: true,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 13469 {
            return Some(DarkOakFenceGate {
                r#open: false,
                r#powered: false,
                r#in_wall: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 13463 {
            return Some(DarkOakFenceGate {
                r#in_wall: false,
                r#powered: false,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 13456 {
            return Some(DarkOakFenceGate {
                r#facing: Facing::South,
                r#powered: true,
                r#open: false,
                r#in_wall: false,
            });
        }
        if state_id == 13446 {
            return Some(DarkOakFenceGate {
                r#facing: Facing::North,
                r#powered: true,
                r#in_wall: false,
                r#open: true,
            });
        }
        if state_id == 13448 {
            return Some(DarkOakFenceGate {
                r#facing: Facing::North,
                r#powered: true,
                r#open: false,
                r#in_wall: false,
            });
        }
        if state_id == 13455 {
            return Some(DarkOakFenceGate {
                r#in_wall: false,
                r#facing: Facing::South,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 13472 {
            return Some(DarkOakFenceGate {
                r#facing: Facing::East,
                r#powered: true,
                r#open: false,
                r#in_wall: false,
            });
        }
        if state_id == 13465 {
            return Some(DarkOakFenceGate {
                r#open: false,
                r#facing: Facing::West,
                r#powered: false,
                r#in_wall: false,
            });
        }
        if state_id == 13458 {
            return Some(DarkOakFenceGate {
                r#facing: Facing::West,
                r#powered: true,
                r#open: true,
                r#in_wall: true,
            });
        }
        if state_id == 13445 {
            return Some(DarkOakFenceGate {
                r#powered: false,
                r#open: false,
                r#facing: Facing::North,
                r#in_wall: true,
            });
        }
        if state_id == 13462 {
            return Some(DarkOakFenceGate {
                r#facing: Facing::West,
                r#open: true,
                r#in_wall: false,
                r#powered: true,
            });
        }
        if state_id == 13457 {
            return Some(DarkOakFenceGate {
                r#powered: false,
                r#facing: Facing::South,
                r#open: false,
                r#in_wall: false,
            });
        }
        if state_id == 13453 {
            return Some(DarkOakFenceGate {
                r#facing: Facing::South,
                r#open: false,
                r#in_wall: true,
                r#powered: false,
            });
        }
        if state_id == 13466 {
            return Some(DarkOakFenceGate {
                r#facing: Facing::East,
                r#open: true,
                r#in_wall: true,
                r#powered: true,
            });
        }
        if state_id == 13452 {
            return Some(DarkOakFenceGate {
                r#facing: Facing::South,
                r#in_wall: true,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 13447 {
            return Some(DarkOakFenceGate {
                r#facing: Facing::North,
                r#in_wall: false,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 13461 {
            return Some(DarkOakFenceGate {
                r#powered: false,
                r#open: false,
                r#facing: Facing::West,
                r#in_wall: true,
            });
        }
        if state_id == 13451 {
            return Some(DarkOakFenceGate {
                r#facing: Facing::South,
                r#powered: false,
                r#open: true,
                r#in_wall: true,
            });
        }
        if state_id == 13473 {
            return Some(DarkOakFenceGate {
                r#in_wall: false,
                r#open: false,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 13468 {
            return Some(DarkOakFenceGate {
                r#facing: Facing::East,
                r#in_wall: true,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 13460 {
            return Some(DarkOakFenceGate {
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
                r#in_wall: true,
            });
        }
        if state_id == 13444 {
            return Some(DarkOakFenceGate {
                r#in_wall: true,
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
            });
        }
        return None;
    }
}

