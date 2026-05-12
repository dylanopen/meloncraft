use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonFenceGate {
    pub in_wall: bool,
    pub powered: bool,
    pub open: bool,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for CrimsonFenceGate {
    fn to_id(self) -> i32 {
        if block_state.r#in_wall == true && block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::West { return 21059; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#in_wall == true { return 21066; }
        if block_state.r#facing == Facing::East && block_state.r#in_wall == true && block_state.r#open == false && block_state.r#powered == false { return 21067; }
        if block_state.r#powered == false && block_state.r#in_wall == false && block_state.r#facing == Facing::West && block_state.r#open == false { return 21063; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#in_wall == false { return 21045; }
        if block_state.r#in_wall == true && block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::South { return 21048; }
        if block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#in_wall == false && block_state.r#powered == true { return 21044; }
        if block_state.r#in_wall == false && block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::North { return 21047; }
        if block_state.r#open == false && block_state.r#in_wall == true && block_state.r#powered == false && block_state.r#facing == Facing::South { return 21051; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#in_wall == true { return 21065; }
        if block_state.r#facing == Facing::North && block_state.r#in_wall == true && block_state.r#powered == true && block_state.r#open == true { return 21040; }
        if block_state.r#powered == false && block_state.r#in_wall == true && block_state.r#open == true && block_state.r#facing == Facing::North { return 21041; }
        if block_state.r#in_wall == false && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#open == false { return 21055; }
        if block_state.r#in_wall == true && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#open == false { return 21043; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#open == false && block_state.r#in_wall == false { return 21070; }
        if block_state.r#powered == true && block_state.r#open == true && block_state.r#in_wall == true && block_state.r#facing == Facing::West { return 21056; }
        if block_state.r#open == true && block_state.r#in_wall == false && block_state.r#facing == Facing::South && block_state.r#powered == true { return 21052; }
        if block_state.r#open == true && block_state.r#in_wall == true && block_state.r#powered == false && block_state.r#facing == Facing::South { return 21049; }
        if block_state.r#in_wall == false && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#powered == true { return 21054; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#in_wall == true && block_state.r#open == false { return 21058; }
        if block_state.r#powered == true && block_state.r#in_wall == true && block_state.r#open == false && block_state.r#facing == Facing::North { return 21042; }
        if block_state.r#in_wall == false && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#powered == false { return 21053; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#in_wall == false && block_state.r#facing == Facing::West { return 21060; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#open == false && block_state.r#in_wall == false { return 21062; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#powered == true && block_state.r#in_wall == false { return 21068; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#in_wall == false && block_state.r#open == true { return 21069; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#in_wall == false && block_state.r#open == false { return 21071; }
        if block_state.r#open == true && block_state.r#in_wall == false && block_state.r#powered == false && block_state.r#facing == Facing::West { return 21061; }
        if block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#in_wall == true { return 21050; }
        if block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#in_wall == false && block_state.r#powered == true { return 21046; }
        if block_state.r#powered == true && block_state.r#in_wall == true && block_state.r#facing == Facing::East && block_state.r#open == true { return 21064; }
        if block_state.r#in_wall == true && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#powered == false { return 21057; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21059 {
            return Some(CrimsonFenceGate {
                r#in_wall: true,
                r#powered: false,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 21066 {
            return Some(CrimsonFenceGate {
                r#open: false,
                r#powered: true,
                r#facing: Facing::East,
                r#in_wall: true,
            });
        }
        if state_id == 21067 {
            return Some(CrimsonFenceGate {
                r#facing: Facing::East,
                r#in_wall: true,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 21063 {
            return Some(CrimsonFenceGate {
                r#powered: false,
                r#in_wall: false,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 21045 {
            return Some(CrimsonFenceGate {
                r#powered: false,
                r#facing: Facing::North,
                r#open: true,
                r#in_wall: false,
            });
        }
        if state_id == 21048 {
            return Some(CrimsonFenceGate {
                r#in_wall: true,
                r#powered: true,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 21044 {
            return Some(CrimsonFenceGate {
                r#facing: Facing::North,
                r#open: true,
                r#in_wall: false,
                r#powered: true,
            });
        }
        if state_id == 21047 {
            return Some(CrimsonFenceGate {
                r#in_wall: false,
                r#powered: false,
                r#open: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 21051 {
            return Some(CrimsonFenceGate {
                r#open: false,
                r#in_wall: true,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 21065 {
            return Some(CrimsonFenceGate {
                r#open: true,
                r#powered: false,
                r#facing: Facing::East,
                r#in_wall: true,
            });
        }
        if state_id == 21040 {
            return Some(CrimsonFenceGate {
                r#facing: Facing::North,
                r#in_wall: true,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 21041 {
            return Some(CrimsonFenceGate {
                r#powered: false,
                r#in_wall: true,
                r#open: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 21055 {
            return Some(CrimsonFenceGate {
                r#in_wall: false,
                r#facing: Facing::South,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 21043 {
            return Some(CrimsonFenceGate {
                r#in_wall: true,
                r#facing: Facing::North,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 21070 {
            return Some(CrimsonFenceGate {
                r#facing: Facing::East,
                r#powered: true,
                r#open: false,
                r#in_wall: false,
            });
        }
        if state_id == 21056 {
            return Some(CrimsonFenceGate {
                r#powered: true,
                r#open: true,
                r#in_wall: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 21052 {
            return Some(CrimsonFenceGate {
                r#open: true,
                r#in_wall: false,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 21049 {
            return Some(CrimsonFenceGate {
                r#open: true,
                r#in_wall: true,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 21054 {
            return Some(CrimsonFenceGate {
                r#in_wall: false,
                r#open: false,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 21058 {
            return Some(CrimsonFenceGate {
                r#facing: Facing::West,
                r#powered: true,
                r#in_wall: true,
                r#open: false,
            });
        }
        if state_id == 21042 {
            return Some(CrimsonFenceGate {
                r#powered: true,
                r#in_wall: true,
                r#open: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 21053 {
            return Some(CrimsonFenceGate {
                r#in_wall: false,
                r#open: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 21060 {
            return Some(CrimsonFenceGate {
                r#open: true,
                r#powered: true,
                r#in_wall: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 21062 {
            return Some(CrimsonFenceGate {
                r#facing: Facing::West,
                r#powered: true,
                r#open: false,
                r#in_wall: false,
            });
        }
        if state_id == 21068 {
            return Some(CrimsonFenceGate {
                r#facing: Facing::East,
                r#open: true,
                r#powered: true,
                r#in_wall: false,
            });
        }
        if state_id == 21069 {
            return Some(CrimsonFenceGate {
                r#facing: Facing::East,
                r#powered: false,
                r#in_wall: false,
                r#open: true,
            });
        }
        if state_id == 21071 {
            return Some(CrimsonFenceGate {
                r#facing: Facing::East,
                r#powered: false,
                r#in_wall: false,
                r#open: false,
            });
        }
        if state_id == 21061 {
            return Some(CrimsonFenceGate {
                r#open: true,
                r#in_wall: false,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 21050 {
            return Some(CrimsonFenceGate {
                r#open: false,
                r#facing: Facing::South,
                r#powered: true,
                r#in_wall: true,
            });
        }
        if state_id == 21046 {
            return Some(CrimsonFenceGate {
                r#facing: Facing::North,
                r#open: false,
                r#in_wall: false,
                r#powered: true,
            });
        }
        if state_id == 21064 {
            return Some(CrimsonFenceGate {
                r#powered: true,
                r#in_wall: true,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 21057 {
            return Some(CrimsonFenceGate {
                r#in_wall: true,
                r#facing: Facing::West,
                r#open: true,
                r#powered: false,
            });
        }
        return None;
    }
}

