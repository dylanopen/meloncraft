use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BambooFenceGate {
    pub r#facing: Facing,
    pub powered: bool,
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

impl BlockState for BambooFenceGate {
    fn to_id(self) -> i32 {
        if block_state.r#powered == true && block_state.r#in_wall == true && block_state.r#facing == Facing::South && block_state.r#open == false { return 13548; }
        if block_state.r#facing == Facing::South && block_state.r#in_wall == false && block_state.r#powered == false && block_state.r#open == true { return 13551; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#in_wall == false && block_state.r#open == false { return 13544; }
        if block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#powered == false && block_state.r#in_wall == true { return 13555; }
        if block_state.r#in_wall == true && block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::North { return 13538; }
        if block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == true && block_state.r#in_wall == false { return 13550; }
        if block_state.r#open == false && block_state.r#in_wall == true && block_state.r#powered == true && block_state.r#facing == Facing::North { return 13540; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#in_wall == true && block_state.r#open == false { return 13541; }
        if block_state.r#in_wall == true && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#open == true { return 13554; }
        if block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#in_wall == false && block_state.r#powered == false { return 13559; }
        if block_state.r#in_wall == true && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::West { return 13557; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#in_wall == true && block_state.r#facing == Facing::East { return 13563; }
        if block_state.r#open == false && block_state.r#in_wall == false && block_state.r#powered == true && block_state.r#facing == Facing::East { return 13568; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#in_wall == false && block_state.r#facing == Facing::East { return 13569; }
        if block_state.r#facing == Facing::North && block_state.r#in_wall == false && block_state.r#open == true && block_state.r#powered == true { return 13542; }
        if block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#in_wall == false && block_state.r#powered == false { return 13567; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#open == true && block_state.r#in_wall == true { return 13546; }
        if block_state.r#in_wall == false && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::South { return 13552; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#in_wall == true && block_state.r#open == false { return 13565; }
        if block_state.r#in_wall == true && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::South { return 13549; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#in_wall == false && block_state.r#facing == Facing::East { return 13566; }
        if block_state.r#powered == false && block_state.r#in_wall == false && block_state.r#facing == Facing::South && block_state.r#open == false { return 13553; }
        if block_state.r#in_wall == true && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#powered == true { return 13564; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#open == false && block_state.r#in_wall == false { return 13545; }
        if block_state.r#in_wall == true && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#open == false { return 13556; }
        if block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#in_wall == true && block_state.r#powered == false { return 13547; }
        if block_state.r#open == false && block_state.r#in_wall == false && block_state.r#facing == Facing::West && block_state.r#powered == true { return 13560; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#open == true && block_state.r#in_wall == true { return 13562; }
        if block_state.r#in_wall == true && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#open == true { return 13539; }
        if block_state.r#facing == Facing::West && block_state.r#in_wall == false && block_state.r#open == true && block_state.r#powered == true { return 13558; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#in_wall == false { return 13543; }
        if block_state.r#open == false && block_state.r#in_wall == false && block_state.r#powered == false && block_state.r#facing == Facing::West { return 13561; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13548 {
            return Some(BambooFenceGate {
                r#powered: true,
                r#in_wall: true,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 13551 {
            return Some(BambooFenceGate {
                r#facing: Facing::South,
                r#in_wall: false,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 13544 {
            return Some(BambooFenceGate {
                r#facing: Facing::North,
                r#powered: true,
                r#in_wall: false,
                r#open: false,
            });
        }
        if state_id == 13555 {
            return Some(BambooFenceGate {
                r#facing: Facing::West,
                r#open: true,
                r#powered: false,
                r#in_wall: true,
            });
        }
        if state_id == 13538 {
            return Some(BambooFenceGate {
                r#in_wall: true,
                r#open: true,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 13550 {
            return Some(BambooFenceGate {
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
                r#in_wall: false,
            });
        }
        if state_id == 13540 {
            return Some(BambooFenceGate {
                r#open: false,
                r#in_wall: true,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 13541 {
            return Some(BambooFenceGate {
                r#facing: Facing::North,
                r#powered: false,
                r#in_wall: true,
                r#open: false,
            });
        }
        if state_id == 13554 {
            return Some(BambooFenceGate {
                r#in_wall: true,
                r#facing: Facing::West,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 13559 {
            return Some(BambooFenceGate {
                r#open: true,
                r#facing: Facing::West,
                r#in_wall: false,
                r#powered: false,
            });
        }
        if state_id == 13557 {
            return Some(BambooFenceGate {
                r#in_wall: true,
                r#open: false,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 13563 {
            return Some(BambooFenceGate {
                r#open: true,
                r#powered: false,
                r#in_wall: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 13568 {
            return Some(BambooFenceGate {
                r#open: false,
                r#in_wall: false,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 13569 {
            return Some(BambooFenceGate {
                r#powered: false,
                r#open: false,
                r#in_wall: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 13542 {
            return Some(BambooFenceGate {
                r#facing: Facing::North,
                r#in_wall: false,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 13567 {
            return Some(BambooFenceGate {
                r#open: true,
                r#facing: Facing::East,
                r#in_wall: false,
                r#powered: false,
            });
        }
        if state_id == 13546 {
            return Some(BambooFenceGate {
                r#facing: Facing::South,
                r#powered: true,
                r#open: true,
                r#in_wall: true,
            });
        }
        if state_id == 13552 {
            return Some(BambooFenceGate {
                r#in_wall: false,
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 13565 {
            return Some(BambooFenceGate {
                r#powered: false,
                r#facing: Facing::East,
                r#in_wall: true,
                r#open: false,
            });
        }
        if state_id == 13549 {
            return Some(BambooFenceGate {
                r#in_wall: true,
                r#open: false,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 13566 {
            return Some(BambooFenceGate {
                r#open: true,
                r#powered: true,
                r#in_wall: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 13553 {
            return Some(BambooFenceGate {
                r#powered: false,
                r#in_wall: false,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 13564 {
            return Some(BambooFenceGate {
                r#in_wall: true,
                r#open: false,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 13545 {
            return Some(BambooFenceGate {
                r#facing: Facing::North,
                r#powered: false,
                r#open: false,
                r#in_wall: false,
            });
        }
        if state_id == 13556 {
            return Some(BambooFenceGate {
                r#in_wall: true,
                r#facing: Facing::West,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 13547 {
            return Some(BambooFenceGate {
                r#facing: Facing::South,
                r#open: true,
                r#in_wall: true,
                r#powered: false,
            });
        }
        if state_id == 13560 {
            return Some(BambooFenceGate {
                r#open: false,
                r#in_wall: false,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 13562 {
            return Some(BambooFenceGate {
                r#facing: Facing::East,
                r#powered: true,
                r#open: true,
                r#in_wall: true,
            });
        }
        if state_id == 13539 {
            return Some(BambooFenceGate {
                r#in_wall: true,
                r#powered: false,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 13558 {
            return Some(BambooFenceGate {
                r#facing: Facing::West,
                r#in_wall: false,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 13543 {
            return Some(BambooFenceGate {
                r#powered: false,
                r#facing: Facing::North,
                r#open: true,
                r#in_wall: false,
            });
        }
        if state_id == 13561 {
            return Some(BambooFenceGate {
                r#open: false,
                r#in_wall: false,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

