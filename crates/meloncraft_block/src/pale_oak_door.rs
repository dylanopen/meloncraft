use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleOakDoor {
    pub powered: bool,
    pub r#hinge: Hinge,
    pub r#half: Half,
    pub r#facing: Facing,
    pub open: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hinge {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Upper,
    Lower,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for PaleOakDoor {
    fn to_id(self) -> i32 {
        if block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South { return 14270; }
        if block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left { return 14283; }
        if block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Lower { return 14303; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower && block_state.r#facing == Facing::East && block_state.r#powered == true { return 14300; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper { return 14258; }
        if block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Upper { return 14262; }
        if block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#powered == false { return 14251; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper { return 14245; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == false { return 14261; }
        if block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#powered == false { return 14281; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#powered == false { return 14305; }
        if block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#powered == true { return 14282; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == true { return 14278; }
        if block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Left && block_state.r#powered == true { return 14268; }
        if block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#powered == false { return 14253; }
        if block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#facing == Facing::North { return 14254; }
        if block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#facing == Facing::North { return 14256; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#half == Half::Upper { return 14280; }
        if block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#hinge == Hinge::Left { return 14252; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#powered == true { return 14276; }
        if block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#half == Half::Upper { return 14263; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Lower { return 14284; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#open == false { return 14272; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::West { return 14275; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#half == Half::Lower { return 14288; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#powered == false { return 14291; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == false { return 14279; }
        if block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Lower { return 14257; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower { return 14250; }
        if block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == false { return 14289; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East && block_state.r#half == Half::Upper && block_state.r#powered == true { return 14296; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::East { return 14304; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#powered == false { return 14299; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#open == false { return 14248; }
        if block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right { return 14286; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#powered == false { return 14255; }
        if block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#hinge == Hinge::Left { return 14269; }
        if block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#powered == false { return 14295; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#powered == false { return 14297; }
        if block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left { return 14290; }
        if block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#open == false { return 14260; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::South { return 14266; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::West && block_state.r#half == Half::Lower { return 14285; }
        if block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#powered == false { return 14247; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#half == Half::Upper { return 14259; }
        if block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#powered == false { return 14249; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left { return 14243; }
        if block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Upper { return 14292; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#half == Half::Upper { return 14277; }
        if block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#open == false { return 14301; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#powered == true { return 14294; }
        if block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#facing == Facing::South { return 14264; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#powered == false { return 14293; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#half == Half::Lower { return 14302; }
        if block_state.r#powered == true && block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::North { return 14246; }
        if block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Lower { return 14298; }
        if block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#hinge == Hinge::Left { return 14242; }
        if block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right { return 14271; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#open == false && block_state.r#hinge == Hinge::Left { return 14244; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#open == false && block_state.r#hinge == Hinge::Right { return 14265; }
        if block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#hinge == Hinge::Right { return 14287; }
        if block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#half == Half::Lower { return 14273; }
        if block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#hinge == Hinge::Left { return 14274; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower { return 14267; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14270 {
            return Some(PaleOakDoor {
                r#powered: true,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
            });
        }
        if state_id == 14283 {
            return Some(PaleOakDoor {
                r#open: true,
                r#half: Half::Lower,
                r#powered: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14303 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Lower,
            });
        }
        if state_id == 14300 {
            return Some(PaleOakDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 14258 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Upper,
            });
        }
        if state_id == 14262 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 14251 {
            return Some(PaleOakDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 14245 {
            return Some(PaleOakDoor {
                r#facing: Facing::North,
                r#powered: false,
                r#open: false,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
            });
        }
        if state_id == 14261 {
            return Some(PaleOakDoor {
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 14281 {
            return Some(PaleOakDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 14305 {
            return Some(PaleOakDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 14282 {
            return Some(PaleOakDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 14278 {
            return Some(PaleOakDoor {
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 14268 {
            return Some(PaleOakDoor {
                r#half: Half::Lower,
                r#open: false,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 14253 {
            return Some(PaleOakDoor {
                r#half: Half::Lower,
                r#open: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 14254 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 14256 {
            return Some(PaleOakDoor {
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 14280 {
            return Some(PaleOakDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 14252 {
            return Some(PaleOakDoor {
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14276 {
            return Some(PaleOakDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 14263 {
            return Some(PaleOakDoor {
                r#open: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 14284 {
            return Some(PaleOakDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Lower,
            });
        }
        if state_id == 14272 {
            return Some(PaleOakDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 14275 {
            return Some(PaleOakDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 14288 {
            return Some(PaleOakDoor {
                r#facing: Facing::West,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 14291 {
            return Some(PaleOakDoor {
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 14279 {
            return Some(PaleOakDoor {
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 14257 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Lower,
            });
        }
        if state_id == 14250 {
            return Some(PaleOakDoor {
                r#powered: true,
                r#facing: Facing::North,
                r#open: true,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
            });
        }
        if state_id == 14289 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 14296 {
            return Some(PaleOakDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 14304 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: false,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 14299 {
            return Some(PaleOakDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 14248 {
            return Some(PaleOakDoor {
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 14286 {
            return Some(PaleOakDoor {
                r#facing: Facing::West,
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14255 {
            return Some(PaleOakDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: false,
            });
        }
        if state_id == 14269 {
            return Some(PaleOakDoor {
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#powered: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14295 {
            return Some(PaleOakDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 14297 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 14290 {
            return Some(PaleOakDoor {
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14260 {
            return Some(PaleOakDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 14266 {
            return Some(PaleOakDoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
            });
        }
        if state_id == 14285 {
            return Some(PaleOakDoor {
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#half: Half::Lower,
            });
        }
        if state_id == 14247 {
            return Some(PaleOakDoor {
                r#open: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#powered: false,
            });
        }
        if state_id == 14259 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Left,
                r#powered: false,
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 14249 {
            return Some(PaleOakDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Upper,
                r#powered: false,
            });
        }
        if state_id == 14243 {
            return Some(PaleOakDoor {
                r#powered: false,
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14292 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#open: false,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 14277 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Left,
                r#powered: false,
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 14301 {
            return Some(PaleOakDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 14294 {
            return Some(PaleOakDoor {
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: true,
            });
        }
        if state_id == 14264 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Right,
                r#powered: true,
                r#open: false,
                r#half: Half::Upper,
                r#facing: Facing::South,
            });
        }
        if state_id == 14293 {
            return Some(PaleOakDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 14302 {
            return Some(PaleOakDoor {
                r#facing: Facing::East,
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 14246 {
            return Some(PaleOakDoor {
                r#powered: true,
                r#open: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 14298 {
            return Some(PaleOakDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 14242 {
            return Some(PaleOakDoor {
                r#facing: Facing::North,
                r#open: true,
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14271 {
            return Some(PaleOakDoor {
                r#half: Half::Lower,
                r#open: true,
                r#powered: false,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14244 {
            return Some(PaleOakDoor {
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14265 {
            return Some(PaleOakDoor {
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#powered: false,
                r#open: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14287 {
            return Some(PaleOakDoor {
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::West,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14273 {
            return Some(PaleOakDoor {
                r#facing: Facing::South,
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 14274 {
            return Some(PaleOakDoor {
                r#half: Half::Upper,
                r#open: true,
                r#facing: Facing::West,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14267 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Lower,
            });
        }
        return None;
    }
}

