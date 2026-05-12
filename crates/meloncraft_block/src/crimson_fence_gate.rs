use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonFenceGate {
    pub r#facing: Facing,
    pub powered: bool,
    pub in_wall: bool,
    pub open: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for CrimsonFenceGate {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South && self.r#powered == true && self.r#open == false && self.r#in_wall == true { return 21050; }
        if self.r#in_wall == false && self.r#open == false && self.r#facing == Facing::South && self.r#powered == true { return 21054; }
        if self.r#in_wall == true && self.r#powered == true && self.r#facing == Facing::North && self.r#open == false { return 21042; }
        if self.r#powered == true && self.r#open == true && self.r#facing == Facing::West && self.r#in_wall == false { return 21060; }
        if self.r#open == true && self.r#facing == Facing::East && self.r#in_wall == true && self.r#powered == false { return 21065; }
        if self.r#facing == Facing::East && self.r#in_wall == false && self.r#open == true && self.r#powered == true { return 21068; }
        if self.r#facing == Facing::East && self.r#open == true && self.r#in_wall == false && self.r#powered == false { return 21069; }
        if self.r#powered == false && self.r#open == true && self.r#facing == Facing::West && self.r#in_wall == true { return 21057; }
        if self.r#facing == Facing::North && self.r#in_wall == true && self.r#powered == false && self.r#open == true { return 21041; }
        if self.r#facing == Facing::South && self.r#in_wall == true && self.r#open == true && self.r#powered == true { return 21048; }
        if self.r#powered == true && self.r#open == true && self.r#facing == Facing::North && self.r#in_wall == true { return 21040; }
        if self.r#in_wall == false && self.r#powered == true && self.r#facing == Facing::North && self.r#open == true { return 21044; }
        if self.r#open == false && self.r#in_wall == false && self.r#facing == Facing::East && self.r#powered == true { return 21070; }
        if self.r#in_wall == false && self.r#facing == Facing::West && self.r#open == true && self.r#powered == false { return 21061; }
        if self.r#open == false && self.r#powered == true && self.r#facing == Facing::West && self.r#in_wall == true { return 21058; }
        if self.r#facing == Facing::East && self.r#in_wall == true && self.r#open == false && self.r#powered == false { return 21067; }
        if self.r#in_wall == true && self.r#powered == false && self.r#open == false && self.r#facing == Facing::South { return 21051; }
        if self.r#in_wall == true && self.r#open == false && self.r#powered == false && self.r#facing == Facing::West { return 21059; }
        if self.r#facing == Facing::South && self.r#open == false && self.r#in_wall == false && self.r#powered == false { return 21055; }
        if self.r#open == false && self.r#facing == Facing::West && self.r#in_wall == false && self.r#powered == false { return 21063; }
        if self.r#in_wall == false && self.r#powered == true && self.r#open == true && self.r#facing == Facing::South { return 21052; }
        if self.r#facing == Facing::East && self.r#in_wall == true && self.r#open == true && self.r#powered == true { return 21064; }
        if self.r#open == false && self.r#facing == Facing::East && self.r#in_wall == true && self.r#powered == true { return 21066; }
        if self.r#open == true && self.r#powered == false && self.r#in_wall == false && self.r#facing == Facing::South { return 21053; }
        if self.r#open == true && self.r#in_wall == true && self.r#facing == Facing::South && self.r#powered == false { return 21049; }
        if self.r#in_wall == false && self.r#open == false && self.r#powered == false && self.r#facing == Facing::East { return 21071; }
        if self.r#powered == true && self.r#facing == Facing::West && self.r#in_wall == false && self.r#open == false { return 21062; }
        if self.r#facing == Facing::North && self.r#in_wall == true && self.r#open == false && self.r#powered == false { return 21043; }
        if self.r#open == false && self.r#in_wall == false && self.r#facing == Facing::North && self.r#powered == true { return 21046; }
        if self.r#open == true && self.r#in_wall == false && self.r#powered == false && self.r#facing == Facing::North { return 21045; }
        if self.r#powered == false && self.r#in_wall == false && self.r#open == false && self.r#facing == Facing::North { return 21047; }
        if self.r#facing == Facing::West && self.r#in_wall == true && self.r#powered == true && self.r#open == true { return 21056; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21050 {
            return Some(CrimsonFenceGate {
                r#facing: Facing::South,
                r#powered: true,
                r#open: false,
                r#in_wall: true,
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
        if state_id == 21042 {
            return Some(CrimsonFenceGate {
                r#in_wall: true,
                r#powered: true,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 21060 {
            return Some(CrimsonFenceGate {
                r#powered: true,
                r#open: true,
                r#facing: Facing::West,
                r#in_wall: false,
            });
        }
        if state_id == 21065 {
            return Some(CrimsonFenceGate {
                r#open: true,
                r#facing: Facing::East,
                r#in_wall: true,
                r#powered: false,
            });
        }
        if state_id == 21068 {
            return Some(CrimsonFenceGate {
                r#facing: Facing::East,
                r#in_wall: false,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 21069 {
            return Some(CrimsonFenceGate {
                r#facing: Facing::East,
                r#open: true,
                r#in_wall: false,
                r#powered: false,
            });
        }
        if state_id == 21057 {
            return Some(CrimsonFenceGate {
                r#powered: false,
                r#open: true,
                r#facing: Facing::West,
                r#in_wall: true,
            });
        }
        if state_id == 21041 {
            return Some(CrimsonFenceGate {
                r#facing: Facing::North,
                r#in_wall: true,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 21048 {
            return Some(CrimsonFenceGate {
                r#facing: Facing::South,
                r#in_wall: true,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 21040 {
            return Some(CrimsonFenceGate {
                r#powered: true,
                r#open: true,
                r#facing: Facing::North,
                r#in_wall: true,
            });
        }
        if state_id == 21044 {
            return Some(CrimsonFenceGate {
                r#in_wall: false,
                r#powered: true,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 21070 {
            return Some(CrimsonFenceGate {
                r#open: false,
                r#in_wall: false,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 21061 {
            return Some(CrimsonFenceGate {
                r#in_wall: false,
                r#facing: Facing::West,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 21058 {
            return Some(CrimsonFenceGate {
                r#open: false,
                r#powered: true,
                r#facing: Facing::West,
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
        if state_id == 21051 {
            return Some(CrimsonFenceGate {
                r#in_wall: true,
                r#powered: false,
                r#open: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 21059 {
            return Some(CrimsonFenceGate {
                r#in_wall: true,
                r#open: false,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 21055 {
            return Some(CrimsonFenceGate {
                r#facing: Facing::South,
                r#open: false,
                r#in_wall: false,
                r#powered: false,
            });
        }
        if state_id == 21063 {
            return Some(CrimsonFenceGate {
                r#open: false,
                r#facing: Facing::West,
                r#in_wall: false,
                r#powered: false,
            });
        }
        if state_id == 21052 {
            return Some(CrimsonFenceGate {
                r#in_wall: false,
                r#powered: true,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 21064 {
            return Some(CrimsonFenceGate {
                r#facing: Facing::East,
                r#in_wall: true,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 21066 {
            return Some(CrimsonFenceGate {
                r#open: false,
                r#facing: Facing::East,
                r#in_wall: true,
                r#powered: true,
            });
        }
        if state_id == 21053 {
            return Some(CrimsonFenceGate {
                r#open: true,
                r#powered: false,
                r#in_wall: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 21049 {
            return Some(CrimsonFenceGate {
                r#open: true,
                r#in_wall: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 21071 {
            return Some(CrimsonFenceGate {
                r#in_wall: false,
                r#open: false,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 21062 {
            return Some(CrimsonFenceGate {
                r#powered: true,
                r#facing: Facing::West,
                r#in_wall: false,
                r#open: false,
            });
        }
        if state_id == 21043 {
            return Some(CrimsonFenceGate {
                r#facing: Facing::North,
                r#in_wall: true,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 21046 {
            return Some(CrimsonFenceGate {
                r#open: false,
                r#in_wall: false,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 21045 {
            return Some(CrimsonFenceGate {
                r#open: true,
                r#in_wall: false,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 21047 {
            return Some(CrimsonFenceGate {
                r#powered: false,
                r#in_wall: false,
                r#open: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 21056 {
            return Some(CrimsonFenceGate {
                r#facing: Facing::West,
                r#in_wall: true,
                r#powered: true,
                r#open: true,
            });
        }
        return None;
    }
}

