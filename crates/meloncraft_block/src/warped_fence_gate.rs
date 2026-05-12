use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedFenceGate {
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

impl BlockState for WarpedFenceGate {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West && self.r#open == false && self.r#powered == false && self.r#in_wall == true { return 21091; }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#in_wall == false && self.r#open == false { return 21086; }
        if self.r#in_wall == false && self.r#facing == Facing::South && self.r#open == true && self.r#powered == true { return 21084; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#in_wall == true && self.r#open == false { return 21083; }
        if self.r#facing == Facing::North && self.r#open == true && self.r#powered == true && self.r#in_wall == true { return 21072; }
        if self.r#in_wall == true && self.r#open == true && self.r#facing == Facing::East && self.r#powered == false { return 21097; }
        if self.r#facing == Facing::West && self.r#open == true && self.r#powered == true && self.r#in_wall == true { return 21088; }
        if self.r#open == false && self.r#in_wall == true && self.r#facing == Facing::East && self.r#powered == false { return 21099; }
        if self.r#facing == Facing::South && self.r#in_wall == true && self.r#open == false && self.r#powered == true { return 21082; }
        if self.r#in_wall == true && self.r#powered == true && self.r#facing == Facing::East && self.r#open == true { return 21096; }
        if self.r#facing == Facing::North && self.r#in_wall == false && self.r#open == false && self.r#powered == true { return 21078; }
        if self.r#powered == false && self.r#facing == Facing::North && self.r#in_wall == false && self.r#open == false { return 21079; }
        if self.r#in_wall == false && self.r#powered == false && self.r#facing == Facing::East && self.r#open == true { return 21101; }
        if self.r#facing == Facing::North && self.r#in_wall == false && self.r#open == true && self.r#powered == false { return 21077; }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#in_wall == true && self.r#open == true { return 21080; }
        if self.r#open == false && self.r#in_wall == false && self.r#powered == false && self.r#facing == Facing::East { return 21103; }
        if self.r#in_wall == false && self.r#open == true && self.r#powered == false && self.r#facing == Facing::West { return 21093; }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#open == true && self.r#in_wall == false { return 21076; }
        if self.r#facing == Facing::North && self.r#open == false && self.r#powered == false && self.r#in_wall == true { return 21075; }
        if self.r#open == true && self.r#facing == Facing::West && self.r#powered == true && self.r#in_wall == false { return 21092; }
        if self.r#in_wall == false && self.r#open == false && self.r#powered == true && self.r#facing == Facing::West { return 21094; }
        if self.r#open == false && self.r#in_wall == false && self.r#facing == Facing::East && self.r#powered == true { return 21102; }
        if self.r#facing == Facing::South && self.r#open == true && self.r#powered == false && self.r#in_wall == false { return 21085; }
        if self.r#in_wall == false && self.r#powered == true && self.r#facing == Facing::East && self.r#open == true { return 21100; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#in_wall == true && self.r#open == false { return 21090; }
        if self.r#in_wall == true && self.r#open == false && self.r#powered == true && self.r#facing == Facing::East { return 21098; }
        if self.r#powered == false && self.r#facing == Facing::West && self.r#open == true && self.r#in_wall == true { return 21089; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#in_wall == false && self.r#open == false { return 21087; }
        if self.r#facing == Facing::South && self.r#in_wall == true && self.r#powered == false && self.r#open == true { return 21081; }
        if self.r#in_wall == false && self.r#powered == false && self.r#open == false && self.r#facing == Facing::West { return 21095; }
        if self.r#open == true && self.r#facing == Facing::North && self.r#in_wall == true && self.r#powered == false { return 21073; }
        if self.r#in_wall == true && self.r#facing == Facing::North && self.r#open == false && self.r#powered == true { return 21074; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21091 {
            return Some(WarpedFenceGate {
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
                r#in_wall: true,
            });
        }
        if state_id == 21086 {
            return Some(WarpedFenceGate {
                r#powered: true,
                r#facing: Facing::South,
                r#in_wall: false,
                r#open: false,
            });
        }
        if state_id == 21084 {
            return Some(WarpedFenceGate {
                r#in_wall: false,
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 21083 {
            return Some(WarpedFenceGate {
                r#powered: false,
                r#facing: Facing::South,
                r#in_wall: true,
                r#open: false,
            });
        }
        if state_id == 21072 {
            return Some(WarpedFenceGate {
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
                r#in_wall: true,
            });
        }
        if state_id == 21097 {
            return Some(WarpedFenceGate {
                r#in_wall: true,
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 21088 {
            return Some(WarpedFenceGate {
                r#facing: Facing::West,
                r#open: true,
                r#powered: true,
                r#in_wall: true,
            });
        }
        if state_id == 21099 {
            return Some(WarpedFenceGate {
                r#open: false,
                r#in_wall: true,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 21082 {
            return Some(WarpedFenceGate {
                r#facing: Facing::South,
                r#in_wall: true,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 21096 {
            return Some(WarpedFenceGate {
                r#in_wall: true,
                r#powered: true,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 21078 {
            return Some(WarpedFenceGate {
                r#facing: Facing::North,
                r#in_wall: false,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 21079 {
            return Some(WarpedFenceGate {
                r#powered: false,
                r#facing: Facing::North,
                r#in_wall: false,
                r#open: false,
            });
        }
        if state_id == 21101 {
            return Some(WarpedFenceGate {
                r#in_wall: false,
                r#powered: false,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 21077 {
            return Some(WarpedFenceGate {
                r#facing: Facing::North,
                r#in_wall: false,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 21080 {
            return Some(WarpedFenceGate {
                r#powered: true,
                r#facing: Facing::South,
                r#in_wall: true,
                r#open: true,
            });
        }
        if state_id == 21103 {
            return Some(WarpedFenceGate {
                r#open: false,
                r#in_wall: false,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 21093 {
            return Some(WarpedFenceGate {
                r#in_wall: false,
                r#open: true,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 21076 {
            return Some(WarpedFenceGate {
                r#facing: Facing::North,
                r#powered: true,
                r#open: true,
                r#in_wall: false,
            });
        }
        if state_id == 21075 {
            return Some(WarpedFenceGate {
                r#facing: Facing::North,
                r#open: false,
                r#powered: false,
                r#in_wall: true,
            });
        }
        if state_id == 21092 {
            return Some(WarpedFenceGate {
                r#open: true,
                r#facing: Facing::West,
                r#powered: true,
                r#in_wall: false,
            });
        }
        if state_id == 21094 {
            return Some(WarpedFenceGate {
                r#in_wall: false,
                r#open: false,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 21102 {
            return Some(WarpedFenceGate {
                r#open: false,
                r#in_wall: false,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 21085 {
            return Some(WarpedFenceGate {
                r#facing: Facing::South,
                r#open: true,
                r#powered: false,
                r#in_wall: false,
            });
        }
        if state_id == 21100 {
            return Some(WarpedFenceGate {
                r#in_wall: false,
                r#powered: true,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 21090 {
            return Some(WarpedFenceGate {
                r#facing: Facing::West,
                r#powered: true,
                r#in_wall: true,
                r#open: false,
            });
        }
        if state_id == 21098 {
            return Some(WarpedFenceGate {
                r#in_wall: true,
                r#open: false,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 21089 {
            return Some(WarpedFenceGate {
                r#powered: false,
                r#facing: Facing::West,
                r#open: true,
                r#in_wall: true,
            });
        }
        if state_id == 21087 {
            return Some(WarpedFenceGate {
                r#powered: false,
                r#facing: Facing::South,
                r#in_wall: false,
                r#open: false,
            });
        }
        if state_id == 21081 {
            return Some(WarpedFenceGate {
                r#facing: Facing::South,
                r#in_wall: true,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 21095 {
            return Some(WarpedFenceGate {
                r#in_wall: false,
                r#powered: false,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 21073 {
            return Some(WarpedFenceGate {
                r#open: true,
                r#facing: Facing::North,
                r#in_wall: true,
                r#powered: false,
            });
        }
        if state_id == 21074 {
            return Some(WarpedFenceGate {
                r#in_wall: true,
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
            });
        }
        return None;
    }
}

