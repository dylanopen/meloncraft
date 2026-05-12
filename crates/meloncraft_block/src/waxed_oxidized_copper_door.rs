use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedOxidizedCopperDoor {
    pub r#facing: Facing,
    pub open: bool,
    pub r#half: Half,
    pub r#hinge: Hinge,
    pub powered: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Upper,
    Lower,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hinge {
    Left,
    Right,
}

impl BlockState for WaxedOxidizedCopperDoor {
    fn to_id(self) -> i32 {
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == false { return 26248; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower { return 26250; }
        if block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left && block_state.r#powered == false { return 26240; }
        if block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#open == true { return 26261; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#open == false { return 26220; }
        if block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#facing == Facing::North { return 26218; }
        if block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Lower { return 26247; }
        if block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#open == true { return 26262; }
        if block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::East && block_state.r#open == false { return 26268; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper { return 26225; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::South { return 26226; }
        if block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Upper { return 26209; }
        if block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == true { return 26207; }
        if block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#hinge == Hinge::Right { return 26217; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#facing == Facing::West { return 26237; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == false { return 26246; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower { return 26230; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#facing == Facing::East { return 26255; }
        if block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#powered == true { return 26221; }
        if block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#powered == true { return 26243; }
        if block_state.r#powered == true && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#half == Half::Lower { return 26213; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#facing == Facing::South { return 26231; }
        if block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::South && block_state.r#powered == true { return 26235; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#powered == false && block_state.r#hinge == Hinge::Right { return 26242; }
        if block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#open == false { return 26219; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Upper { return 26254; }
        if block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#open == false { return 26260; }
        if block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#half == Half::Upper { return 26258; }
        if block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#hinge == Hinge::Left { return 26206; }
        if block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#half == Half::Upper { return 26223; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#half == Half::Lower { return 26264; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#hinge == Hinge::Right { return 26267; }
        if block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::West && block_state.r#open == false { return 26252; }
        if block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::North && block_state.r#open == false { return 26211; }
        if block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right { return 26212; }
        if block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper { return 26244; }
        if block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#powered == false { return 26222; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#open == false { return 26263; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#facing == Facing::West { return 26251; }
        if block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right { return 26234; }
        if block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#facing == Facing::South { return 26227; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#powered == false { return 26216; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#open == false { return 26228; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#powered == false && block_state.r#hinge == Hinge::Right { return 26236; }
        if block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#half == Half::Upper { return 26238; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#open == false { return 26232; }
        if block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right && block_state.r#open == true { return 26233; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East { return 26253; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#powered == true && block_state.r#hinge == Hinge::Left { return 26229; }
        if block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#half == Half::Upper { return 26241; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#open == true { return 26257; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right { return 26266; }
        if block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#facing == Facing::North { return 26210; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#open == true { return 26245; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper { return 26256; }
        if block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#powered == true { return 26265; }
        if block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#open == true && block_state.r#half == Half::Lower { return 26249; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#half == Half::Lower { return 26214; }
        if block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left { return 26208; }
        if block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#powered == true { return 26239; }
        if block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#powered == true { return 26205; }
        if block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower && block_state.r#powered == true { return 26215; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#facing == Facing::South { return 26224; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#half == Half::Upper { return 26259; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26248 {
            return Some(WaxedOxidizedCopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26250 {
            return Some(WaxedOxidizedCopperDoor {
                r#facing: Facing::West,
                r#powered: false,
                r#open: true,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
            });
        }
        if state_id == 26240 {
            return Some(WaxedOxidizedCopperDoor {
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 26261 {
            return Some(WaxedOxidizedCopperDoor {
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 26220 {
            return Some(WaxedOxidizedCopperDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 26218 {
            return Some(WaxedOxidizedCopperDoor {
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26247 {
            return Some(WaxedOxidizedCopperDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 26262 {
            return Some(WaxedOxidizedCopperDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 26268 {
            return Some(WaxedOxidizedCopperDoor {
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 26225 {
            return Some(WaxedOxidizedCopperDoor {
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Upper,
            });
        }
        if state_id == 26226 {
            return Some(WaxedOxidizedCopperDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#open: true,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 26209 {
            return Some(WaxedOxidizedCopperDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 26207 {
            return Some(WaxedOxidizedCopperDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26217 {
            return Some(WaxedOxidizedCopperDoor {
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::North,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26237 {
            return Some(WaxedOxidizedCopperDoor {
                r#powered: true,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 26246 {
            return Some(WaxedOxidizedCopperDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 26230 {
            return Some(WaxedOxidizedCopperDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Lower,
            });
        }
        if state_id == 26255 {
            return Some(WaxedOxidizedCopperDoor {
                r#powered: true,
                r#hinge: Hinge::Left,
                r#open: false,
                r#half: Half::Upper,
                r#facing: Facing::East,
            });
        }
        if state_id == 26221 {
            return Some(WaxedOxidizedCopperDoor {
                r#facing: Facing::South,
                r#open: true,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 26243 {
            return Some(WaxedOxidizedCopperDoor {
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#powered: true,
            });
        }
        if state_id == 26213 {
            return Some(WaxedOxidizedCopperDoor {
                r#powered: true,
                r#open: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#half: Half::Lower,
            });
        }
        if state_id == 26231 {
            return Some(WaxedOxidizedCopperDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 26235 {
            return Some(WaxedOxidizedCopperDoor {
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 26242 {
            return Some(WaxedOxidizedCopperDoor {
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26219 {
            return Some(WaxedOxidizedCopperDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 26254 {
            return Some(WaxedOxidizedCopperDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Upper,
            });
        }
        if state_id == 26260 {
            return Some(WaxedOxidizedCopperDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Upper,
                r#open: false,
            });
        }
        if state_id == 26258 {
            return Some(WaxedOxidizedCopperDoor {
                r#open: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 26206 {
            return Some(WaxedOxidizedCopperDoor {
                r#half: Half::Upper,
                r#open: true,
                r#facing: Facing::North,
                r#powered: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26223 {
            return Some(WaxedOxidizedCopperDoor {
                r#facing: Facing::South,
                r#open: false,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 26264 {
            return Some(WaxedOxidizedCopperDoor {
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#half: Half::Lower,
            });
        }
        if state_id == 26267 {
            return Some(WaxedOxidizedCopperDoor {
                r#facing: Facing::East,
                r#powered: true,
                r#half: Half::Lower,
                r#open: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26252 {
            return Some(WaxedOxidizedCopperDoor {
                r#half: Half::Lower,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 26211 {
            return Some(WaxedOxidizedCopperDoor {
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 26212 {
            return Some(WaxedOxidizedCopperDoor {
                r#half: Half::Upper,
                r#open: false,
                r#powered: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26244 {
            return Some(WaxedOxidizedCopperDoor {
                r#open: false,
                r#facing: Facing::West,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
            });
        }
        if state_id == 26222 {
            return Some(WaxedOxidizedCopperDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 26263 {
            return Some(WaxedOxidizedCopperDoor {
                r#powered: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 26251 {
            return Some(WaxedOxidizedCopperDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 26234 {
            return Some(WaxedOxidizedCopperDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26227 {
            return Some(WaxedOxidizedCopperDoor {
                r#open: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 26216 {
            return Some(WaxedOxidizedCopperDoor {
                r#hinge: Hinge::Left,
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 26228 {
            return Some(WaxedOxidizedCopperDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#powered: false,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 26236 {
            return Some(WaxedOxidizedCopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26238 {
            return Some(WaxedOxidizedCopperDoor {
                r#facing: Facing::West,
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 26232 {
            return Some(WaxedOxidizedCopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 26233 {
            return Some(WaxedOxidizedCopperDoor {
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 26253 {
            return Some(WaxedOxidizedCopperDoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 26229 {
            return Some(WaxedOxidizedCopperDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26241 {
            return Some(WaxedOxidizedCopperDoor {
                r#facing: Facing::West,
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 26257 {
            return Some(WaxedOxidizedCopperDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 26266 {
            return Some(WaxedOxidizedCopperDoor {
                r#open: true,
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26210 {
            return Some(WaxedOxidizedCopperDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26245 {
            return Some(WaxedOxidizedCopperDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 26256 {
            return Some(WaxedOxidizedCopperDoor {
                r#open: false,
                r#powered: false,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
            });
        }
        if state_id == 26265 {
            return Some(WaxedOxidizedCopperDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26249 {
            return Some(WaxedOxidizedCopperDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#open: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 26214 {
            return Some(WaxedOxidizedCopperDoor {
                r#hinge: Hinge::Left,
                r#powered: false,
                r#facing: Facing::North,
                r#open: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 26208 {
            return Some(WaxedOxidizedCopperDoor {
                r#half: Half::Upper,
                r#open: false,
                r#powered: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26239 {
            return Some(WaxedOxidizedCopperDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26205 {
            return Some(WaxedOxidizedCopperDoor {
                r#facing: Facing::North,
                r#open: true,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 26215 {
            return Some(WaxedOxidizedCopperDoor {
                r#open: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#powered: true,
            });
        }
        if state_id == 26224 {
            return Some(WaxedOxidizedCopperDoor {
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: false,
                r#half: Half::Upper,
                r#facing: Facing::South,
            });
        }
        if state_id == 26259 {
            return Some(WaxedOxidizedCopperDoor {
                r#facing: Facing::East,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Upper,
            });
        }
        return None;
    }
}

