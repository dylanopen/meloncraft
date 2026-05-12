use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherryFenceGate {
    pub powered: bool,
    pub open: bool,
    pub r#facing: Facing,
    pub in_wall: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for CherryFenceGate {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::West && block_state.r#in_wall == false && block_state.r#powered == true && block_state.r#open == true { return 13430; }
        if block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#in_wall == false { return 13431; }
        if block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#in_wall == false && block_state.r#powered == true { return 13432; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#in_wall == false { return 13439; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#in_wall == true && block_state.r#open == true { return 13426; }
        if block_state.r#in_wall == false && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#powered == false { return 13425; }
        if block_state.r#facing == Facing::East && block_state.r#in_wall == true && block_state.r#open == false && block_state.r#powered == false { return 13437; }
        if block_state.r#facing == Facing::East && block_state.r#in_wall == false && block_state.r#powered == true && block_state.r#open == false { return 13440; }
        if block_state.r#in_wall == false && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == true { return 13422; }
        if block_state.r#powered == true && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#in_wall == true { return 13436; }
        if block_state.r#open == false && block_state.r#in_wall == true && block_state.r#facing == Facing::South && block_state.r#powered == false { return 13421; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#in_wall == true && block_state.r#facing == Facing::East { return 13435; }
        if block_state.r#open == true && block_state.r#in_wall == false && block_state.r#facing == Facing::South && block_state.r#powered == false { return 13423; }
        if block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#powered == true && block_state.r#in_wall == false { return 13424; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#in_wall == true && block_state.r#facing == Facing::West { return 13429; }
        if block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#in_wall == false { return 13414; }
        if block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#in_wall == false && block_state.r#powered == false { return 13415; }
        if block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#in_wall == false && block_state.r#powered == false { return 13417; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#in_wall == true { return 13418; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#in_wall == true && block_state.r#open == true { return 13419; }
        if block_state.r#in_wall == true && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#powered == true { return 13420; }
        if block_state.r#facing == Facing::North && block_state.r#in_wall == true && block_state.r#powered == false && block_state.r#open == true { return 13411; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#open == false && block_state.r#in_wall == true { return 13428; }
        if block_state.r#in_wall == true && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#powered == true { return 13410; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#in_wall == false && block_state.r#open == true { return 13438; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#in_wall == true && block_state.r#facing == Facing::West { return 13427; }
        if block_state.r#in_wall == true && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == true { return 13434; }
        if block_state.r#facing == Facing::North && block_state.r#in_wall == true && block_state.r#powered == true && block_state.r#open == false { return 13412; }
        if block_state.r#in_wall == false && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::East { return 13441; }
        if block_state.r#facing == Facing::North && block_state.r#in_wall == false && block_state.r#powered == true && block_state.r#open == false { return 13416; }
        if block_state.r#open == false && block_state.r#in_wall == false && block_state.r#powered == false && block_state.r#facing == Facing::West { return 13433; }
        if block_state.r#facing == Facing::North && block_state.r#in_wall == true && block_state.r#powered == false && block_state.r#open == false { return 13413; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13430 {
            return Some(CherryFenceGate {
                r#facing: Facing::West,
                r#in_wall: false,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 13431 {
            return Some(CherryFenceGate {
                r#open: true,
                r#facing: Facing::West,
                r#powered: false,
                r#in_wall: false,
            });
        }
        if state_id == 13432 {
            return Some(CherryFenceGate {
                r#open: false,
                r#facing: Facing::West,
                r#in_wall: false,
                r#powered: true,
            });
        }
        if state_id == 13439 {
            return Some(CherryFenceGate {
                r#open: true,
                r#powered: false,
                r#facing: Facing::East,
                r#in_wall: false,
            });
        }
        if state_id == 13426 {
            return Some(CherryFenceGate {
                r#powered: true,
                r#facing: Facing::West,
                r#in_wall: true,
                r#open: true,
            });
        }
        if state_id == 13425 {
            return Some(CherryFenceGate {
                r#in_wall: false,
                r#facing: Facing::South,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 13437 {
            return Some(CherryFenceGate {
                r#facing: Facing::East,
                r#in_wall: true,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 13440 {
            return Some(CherryFenceGate {
                r#facing: Facing::East,
                r#in_wall: false,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 13422 {
            return Some(CherryFenceGate {
                r#in_wall: false,
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 13436 {
            return Some(CherryFenceGate {
                r#powered: true,
                r#open: false,
                r#facing: Facing::East,
                r#in_wall: true,
            });
        }
        if state_id == 13421 {
            return Some(CherryFenceGate {
                r#open: false,
                r#in_wall: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 13435 {
            return Some(CherryFenceGate {
                r#powered: false,
                r#open: true,
                r#in_wall: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 13423 {
            return Some(CherryFenceGate {
                r#open: true,
                r#in_wall: false,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 13424 {
            return Some(CherryFenceGate {
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
                r#in_wall: false,
            });
        }
        if state_id == 13429 {
            return Some(CherryFenceGate {
                r#powered: false,
                r#open: false,
                r#in_wall: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 13414 {
            return Some(CherryFenceGate {
                r#powered: true,
                r#open: true,
                r#facing: Facing::North,
                r#in_wall: false,
            });
        }
        if state_id == 13415 {
            return Some(CherryFenceGate {
                r#open: true,
                r#facing: Facing::North,
                r#in_wall: false,
                r#powered: false,
            });
        }
        if state_id == 13417 {
            return Some(CherryFenceGate {
                r#facing: Facing::North,
                r#open: false,
                r#in_wall: false,
                r#powered: false,
            });
        }
        if state_id == 13418 {
            return Some(CherryFenceGate {
                r#open: true,
                r#powered: true,
                r#facing: Facing::South,
                r#in_wall: true,
            });
        }
        if state_id == 13419 {
            return Some(CherryFenceGate {
                r#facing: Facing::South,
                r#powered: false,
                r#in_wall: true,
                r#open: true,
            });
        }
        if state_id == 13420 {
            return Some(CherryFenceGate {
                r#in_wall: true,
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 13411 {
            return Some(CherryFenceGate {
                r#facing: Facing::North,
                r#in_wall: true,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 13428 {
            return Some(CherryFenceGate {
                r#facing: Facing::West,
                r#powered: true,
                r#open: false,
                r#in_wall: true,
            });
        }
        if state_id == 13410 {
            return Some(CherryFenceGate {
                r#in_wall: true,
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 13438 {
            return Some(CherryFenceGate {
                r#facing: Facing::East,
                r#powered: true,
                r#in_wall: false,
                r#open: true,
            });
        }
        if state_id == 13427 {
            return Some(CherryFenceGate {
                r#powered: false,
                r#open: true,
                r#in_wall: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 13434 {
            return Some(CherryFenceGate {
                r#in_wall: true,
                r#open: true,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 13412 {
            return Some(CherryFenceGate {
                r#facing: Facing::North,
                r#in_wall: true,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 13441 {
            return Some(CherryFenceGate {
                r#in_wall: false,
                r#open: false,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 13416 {
            return Some(CherryFenceGate {
                r#facing: Facing::North,
                r#in_wall: false,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 13433 {
            return Some(CherryFenceGate {
                r#open: false,
                r#in_wall: false,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 13413 {
            return Some(CherryFenceGate {
                r#facing: Facing::North,
                r#in_wall: true,
                r#powered: false,
                r#open: false,
            });
        }
        return None;
    }
}

