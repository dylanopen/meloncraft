use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherryFenceGate {
    pub powered: bool,
    pub open: bool,
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

impl BlockState for CherryFenceGate {
    fn to_id(&self) -> i32 {
        if self.r#open == false && self.r#facing == Facing::West && self.r#powered == false && self.r#in_wall == true { return 13429; }
        if self.r#in_wall == false && self.r#facing == Facing::East && self.r#powered == false && self.r#open == false { return 13441; }
        if self.r#powered == false && self.r#facing == Facing::East && self.r#open == true && self.r#in_wall == true { return 13435; }
        if self.r#in_wall == true && self.r#facing == Facing::East && self.r#powered == false && self.r#open == false { return 13437; }
        if self.r#open == true && self.r#powered == false && self.r#facing == Facing::South && self.r#in_wall == false { return 13423; }
        if self.r#in_wall == false && self.r#open == true && self.r#powered == true && self.r#facing == Facing::North { return 13414; }
        if self.r#open == true && self.r#powered == false && self.r#in_wall == true && self.r#facing == Facing::South { return 13419; }
        if self.r#open == true && self.r#facing == Facing::South && self.r#powered == true && self.r#in_wall == false { return 13422; }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#in_wall == false && self.r#open == false { return 13424; }
        if self.r#open == true && self.r#facing == Facing::West && self.r#powered == false && self.r#in_wall == false { return 13431; }
        if self.r#open == false && self.r#facing == Facing::West && self.r#in_wall == false && self.r#powered == false { return 13433; }
        if self.r#facing == Facing::East && self.r#in_wall == false && self.r#open == false && self.r#powered == true { return 13440; }
        if self.r#in_wall == true && self.r#open == true && self.r#facing == Facing::West && self.r#powered == true { return 13426; }
        if self.r#in_wall == false && self.r#facing == Facing::North && self.r#powered == false && self.r#open == true { return 13415; }
        if self.r#facing == Facing::West && self.r#in_wall == true && self.r#powered == true && self.r#open == false { return 13428; }
        if self.r#open == true && self.r#in_wall == true && self.r#facing == Facing::West && self.r#powered == false { return 13427; }
        if self.r#powered == false && self.r#open == false && self.r#in_wall == true && self.r#facing == Facing::North { return 13413; }
        if self.r#in_wall == true && self.r#open == false && self.r#facing == Facing::East && self.r#powered == true { return 13436; }
        if self.r#in_wall == false && self.r#facing == Facing::West && self.r#open == true && self.r#powered == true { return 13430; }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#open == false && self.r#in_wall == true { return 13420; }
        if self.r#facing == Facing::South && self.r#in_wall == false && self.r#open == false && self.r#powered == false { return 13425; }
        if self.r#in_wall == true && self.r#open == false && self.r#powered == false && self.r#facing == Facing::South { return 13421; }
        if self.r#in_wall == false && self.r#facing == Facing::East && self.r#powered == false && self.r#open == true { return 13439; }
        if self.r#in_wall == true && self.r#open == true && self.r#powered == true && self.r#facing == Facing::East { return 13434; }
        if self.r#facing == Facing::North && self.r#in_wall == true && self.r#open == true && self.r#powered == false { return 13411; }
        if self.r#powered == true && self.r#in_wall == true && self.r#facing == Facing::South && self.r#open == true { return 13418; }
        if self.r#in_wall == true && self.r#powered == true && self.r#facing == Facing::North && self.r#open == false { return 13412; }
        if self.r#in_wall == false && self.r#open == false && self.r#facing == Facing::North && self.r#powered == false { return 13417; }
        if self.r#facing == Facing::North && self.r#open == true && self.r#powered == true && self.r#in_wall == true { return 13410; }
        if self.r#open == false && self.r#in_wall == false && self.r#facing == Facing::West && self.r#powered == true { return 13432; }
        if self.r#open == true && self.r#facing == Facing::East && self.r#powered == true && self.r#in_wall == false { return 13438; }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#open == false && self.r#in_wall == false { return 13416; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13429 {
            return Some(CherryFenceGate {
                r#open: false,
                r#facing: Facing::West,
                r#powered: false,
                r#in_wall: true,
            });
        }
        if state_id == 13441 {
            return Some(CherryFenceGate {
                r#in_wall: false,
                r#facing: Facing::East,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 13435 {
            return Some(CherryFenceGate {
                r#powered: false,
                r#facing: Facing::East,
                r#open: true,
                r#in_wall: true,
            });
        }
        if state_id == 13437 {
            return Some(CherryFenceGate {
                r#in_wall: true,
                r#facing: Facing::East,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 13423 {
            return Some(CherryFenceGate {
                r#open: true,
                r#powered: false,
                r#facing: Facing::South,
                r#in_wall: false,
            });
        }
        if state_id == 13414 {
            return Some(CherryFenceGate {
                r#in_wall: false,
                r#open: true,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 13419 {
            return Some(CherryFenceGate {
                r#open: true,
                r#powered: false,
                r#in_wall: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 13422 {
            return Some(CherryFenceGate {
                r#open: true,
                r#facing: Facing::South,
                r#powered: true,
                r#in_wall: false,
            });
        }
        if state_id == 13424 {
            return Some(CherryFenceGate {
                r#powered: true,
                r#facing: Facing::South,
                r#in_wall: false,
                r#open: false,
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
        if state_id == 13433 {
            return Some(CherryFenceGate {
                r#open: false,
                r#facing: Facing::West,
                r#in_wall: false,
                r#powered: false,
            });
        }
        if state_id == 13440 {
            return Some(CherryFenceGate {
                r#facing: Facing::East,
                r#in_wall: false,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 13426 {
            return Some(CherryFenceGate {
                r#in_wall: true,
                r#open: true,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 13415 {
            return Some(CherryFenceGate {
                r#in_wall: false,
                r#facing: Facing::North,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 13428 {
            return Some(CherryFenceGate {
                r#facing: Facing::West,
                r#in_wall: true,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 13427 {
            return Some(CherryFenceGate {
                r#open: true,
                r#in_wall: true,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 13413 {
            return Some(CherryFenceGate {
                r#powered: false,
                r#open: false,
                r#in_wall: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 13436 {
            return Some(CherryFenceGate {
                r#in_wall: true,
                r#open: false,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 13430 {
            return Some(CherryFenceGate {
                r#in_wall: false,
                r#facing: Facing::West,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 13420 {
            return Some(CherryFenceGate {
                r#facing: Facing::South,
                r#powered: true,
                r#open: false,
                r#in_wall: true,
            });
        }
        if state_id == 13425 {
            return Some(CherryFenceGate {
                r#facing: Facing::South,
                r#in_wall: false,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 13421 {
            return Some(CherryFenceGate {
                r#in_wall: true,
                r#open: false,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 13439 {
            return Some(CherryFenceGate {
                r#in_wall: false,
                r#facing: Facing::East,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 13434 {
            return Some(CherryFenceGate {
                r#in_wall: true,
                r#open: true,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 13411 {
            return Some(CherryFenceGate {
                r#facing: Facing::North,
                r#in_wall: true,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 13418 {
            return Some(CherryFenceGate {
                r#powered: true,
                r#in_wall: true,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 13412 {
            return Some(CherryFenceGate {
                r#in_wall: true,
                r#powered: true,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 13417 {
            return Some(CherryFenceGate {
                r#in_wall: false,
                r#open: false,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 13410 {
            return Some(CherryFenceGate {
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
                r#in_wall: true,
            });
        }
        if state_id == 13432 {
            return Some(CherryFenceGate {
                r#open: false,
                r#in_wall: false,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 13438 {
            return Some(CherryFenceGate {
                r#open: true,
                r#facing: Facing::East,
                r#powered: true,
                r#in_wall: false,
            });
        }
        if state_id == 13416 {
            return Some(CherryFenceGate {
                r#facing: Facing::North,
                r#powered: true,
                r#open: false,
                r#in_wall: false,
            });
        }
        return None;
    }
}

