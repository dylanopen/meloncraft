use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleOakDoor {
    pub r#hinge: Hinge,
    pub r#half: Half,
    pub open: bool,
    pub powered: bool,
    pub r#facing: Facing,
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
    fn to_id(&self) -> i32 {
        if self.r#open == true && self.r#half == Half::Lower && self.r#facing == Facing::North && self.r#powered == true && self.r#hinge == Hinge::Left { return 14250; }
        if self.r#hinge == Hinge::Left && self.r#facing == Facing::North && self.r#powered == true && self.r#half == Half::Lower && self.r#open == false { return 14252; }
        if self.r#facing == Facing::West && self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#powered == true && self.r#open == true { return 14282; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Right && self.r#open == false && self.r#facing == Facing::West && self.r#powered == false { return 14281; }
        if self.r#half == Half::Upper && self.r#facing == Facing::West && self.r#hinge == Hinge::Right && self.r#open == false && self.r#powered == true { return 14280; }
        if self.r#facing == Facing::South && self.r#hinge == Hinge::Right && self.r#powered == true && self.r#half == Half::Lower && self.r#open == true { return 14270; }
        if self.r#open == true && self.r#half == Half::Upper && self.r#facing == Facing::West && self.r#powered == true && self.r#hinge == Hinge::Left { return 14274; }
        if self.r#facing == Facing::North && self.r#powered == false && self.r#half == Half::Lower && self.r#hinge == Hinge::Right && self.r#open == false { return 14257; }
        if self.r#half == Half::Lower && self.r#open == true && self.r#hinge == Hinge::Left && self.r#facing == Facing::West && self.r#powered == false { return 14283; }
        if self.r#half == Half::Upper && self.r#facing == Facing::South && self.r#hinge == Hinge::Left && self.r#open == false && self.r#powered == true { return 14260; }
        if self.r#facing == Facing::East && self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#powered == false && self.r#open == false { return 14293; }
        if self.r#half == Half::Upper && self.r#powered == true && self.r#facing == Facing::South && self.r#hinge == Hinge::Right && self.r#open == false { return 14264; }
        if self.r#facing == Facing::East && self.r#open == false && self.r#powered == true && self.r#hinge == Hinge::Right && self.r#half == Half::Upper { return 14296; }
        if self.r#hinge == Hinge::Left && self.r#half == Half::Lower && self.r#open == false && self.r#facing == Facing::East && self.r#powered == false { return 14301; }
        if self.r#half == Half::Lower && self.r#open == false && self.r#powered == true && self.r#facing == Facing::West && self.r#hinge == Hinge::Right { return 14288; }
        if self.r#powered == false && self.r#hinge == Hinge::Left && self.r#facing == Facing::North && self.r#half == Half::Upper && self.r#open == true { return 14243; }
        if self.r#open == true && self.r#powered == false && self.r#facing == Facing::North && self.r#hinge == Hinge::Right && self.r#half == Half::Upper { return 14247; }
        if self.r#hinge == Hinge::Right && self.r#open == true && self.r#facing == Facing::South && self.r#half == Half::Lower && self.r#powered == false { return 14271; }
        if self.r#facing == Facing::West && self.r#powered == false && self.r#hinge == Hinge::Left && self.r#open == false && self.r#half == Half::Upper { return 14277; }
        if self.r#half == Half::Upper && self.r#open == true && self.r#facing == Facing::West && self.r#hinge == Hinge::Left && self.r#powered == false { return 14275; }
        if self.r#powered == true && self.r#hinge == Hinge::Left && self.r#half == Half::Upper && self.r#facing == Facing::East && self.r#open == true { return 14290; }
        if self.r#half == Half::Lower && self.r#hinge == Hinge::Right && self.r#facing == Facing::North && self.r#open == false && self.r#powered == true { return 14256; }
        if self.r#powered == true && self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#facing == Facing::South && self.r#open == true { return 14266; }
        if self.r#facing == Facing::North && self.r#open == false && self.r#powered == true && self.r#hinge == Hinge::Right && self.r#half == Half::Upper { return 14248; }
        if self.r#powered == false && self.r#half == Half::Upper && self.r#open == false && self.r#hinge == Hinge::Right && self.r#facing == Facing::South { return 14265; }
        if self.r#facing == Facing::West && self.r#half == Half::Lower && self.r#powered == true && self.r#open == true && self.r#hinge == Hinge::Right { return 14286; }
        if self.r#open == false && self.r#powered == false && self.r#half == Half::Upper && self.r#hinge == Hinge::Right && self.r#facing == Facing::East { return 14297; }
        if self.r#hinge == Hinge::Left && self.r#half == Half::Lower && self.r#powered == true && self.r#open == false && self.r#facing == Facing::East { return 14300; }
        if self.r#hinge == Hinge::Right && self.r#half == Half::Lower && self.r#open == true && self.r#facing == Facing::East && self.r#powered == false { return 14303; }
        if self.r#hinge == Hinge::Left && self.r#open == true && self.r#half == Half::Upper && self.r#powered == false && self.r#facing == Facing::East { return 14291; }
        if self.r#open == false && self.r#facing == Facing::South && self.r#hinge == Hinge::Left && self.r#half == Half::Lower && self.r#powered == true { return 14268; }
        if self.r#powered == true && self.r#half == Half::Upper && self.r#open == true && self.r#facing == Facing::South && self.r#hinge == Hinge::Left { return 14258; }
        if self.r#powered == false && self.r#half == Half::Lower && self.r#hinge == Hinge::Right && self.r#facing == Facing::East && self.r#open == false { return 14305; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#powered == true && self.r#facing == Facing::East && self.r#open == false { return 14292; }
        if self.r#half == Half::Upper && self.r#facing == Facing::North && self.r#powered == false && self.r#hinge == Hinge::Left && self.r#open == false { return 14245; }
        if self.r#open == false && self.r#hinge == Hinge::Left && self.r#facing == Facing::West && self.r#powered == true && self.r#half == Half::Upper { return 14276; }
        if self.r#open == false && self.r#half == Half::Lower && self.r#powered == false && self.r#facing == Facing::South && self.r#hinge == Hinge::Right { return 14273; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Right && self.r#open == true && self.r#facing == Facing::East && self.r#powered == false { return 14295; }
        if self.r#facing == Facing::East && self.r#half == Half::Lower && self.r#powered == false && self.r#open == true && self.r#hinge == Hinge::Left { return 14299; }
        if self.r#powered == false && self.r#hinge == Hinge::Left && self.r#facing == Facing::North && self.r#half == Half::Lower && self.r#open == false { return 14253; }
        if self.r#powered == false && self.r#open == true && self.r#half == Half::Upper && self.r#facing == Facing::South && self.r#hinge == Hinge::Left { return 14259; }
        if self.r#open == false && self.r#powered == true && self.r#facing == Facing::East && self.r#half == Half::Lower && self.r#hinge == Hinge::Right { return 14304; }
        if self.r#half == Half::Upper && self.r#powered == false && self.r#facing == Facing::West && self.r#hinge == Hinge::Right && self.r#open == true { return 14279; }
        if self.r#hinge == Hinge::Left && self.r#facing == Facing::North && self.r#half == Half::Upper && self.r#open == true && self.r#powered == true { return 14242; }
        if self.r#facing == Facing::South && self.r#open == true && self.r#powered == false && self.r#hinge == Hinge::Left && self.r#half == Half::Lower { return 14267; }
        if self.r#facing == Facing::North && self.r#hinge == Hinge::Left && self.r#open == false && self.r#powered == true && self.r#half == Half::Upper { return 14244; }
        if self.r#half == Half::Lower && self.r#powered == true && self.r#facing == Facing::North && self.r#open == true && self.r#hinge == Hinge::Right { return 14254; }
        if self.r#powered == true && self.r#half == Half::Upper && self.r#facing == Facing::East && self.r#open == true && self.r#hinge == Hinge::Right { return 14294; }
        if self.r#hinge == Hinge::Right && self.r#open == true && self.r#powered == false && self.r#half == Half::Lower && self.r#facing == Facing::West { return 14287; }
        if self.r#powered == false && self.r#facing == Facing::West && self.r#hinge == Hinge::Right && self.r#open == false && self.r#half == Half::Lower { return 14289; }
        if self.r#hinge == Hinge::Right && self.r#facing == Facing::East && self.r#half == Half::Lower && self.r#open == true && self.r#powered == true { return 14302; }
        if self.r#half == Half::Upper && self.r#facing == Facing::North && self.r#hinge == Hinge::Right && self.r#open == true && self.r#powered == true { return 14246; }
        if self.r#half == Half::Upper && self.r#facing == Facing::South && self.r#hinge == Hinge::Right && self.r#powered == true && self.r#open == true { return 14262; }
        if self.r#half == Half::Upper && self.r#facing == Facing::South && self.r#open == true && self.r#hinge == Hinge::Right && self.r#powered == false { return 14263; }
        if self.r#powered == true && self.r#facing == Facing::West && self.r#hinge == Hinge::Right && self.r#open == true && self.r#half == Half::Upper { return 14278; }
        if self.r#facing == Facing::West && self.r#open == false && self.r#powered == true && self.r#half == Half::Lower && self.r#hinge == Hinge::Left { return 14284; }
        if self.r#open == false && self.r#facing == Facing::North && self.r#hinge == Hinge::Right && self.r#half == Half::Upper && self.r#powered == false { return 14249; }
        if self.r#powered == false && self.r#half == Half::Lower && self.r#open == true && self.r#facing == Facing::North && self.r#hinge == Hinge::Right { return 14255; }
        if self.r#half == Half::Lower && self.r#powered == true && self.r#open == true && self.r#hinge == Hinge::Left && self.r#facing == Facing::East { return 14298; }
        if self.r#open == false && self.r#half == Half::Upper && self.r#facing == Facing::South && self.r#hinge == Hinge::Left && self.r#powered == false { return 14261; }
        if self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#open == true && self.r#powered == false && self.r#facing == Facing::North { return 14251; }
        if self.r#facing == Facing::South && self.r#hinge == Hinge::Left && self.r#open == false && self.r#half == Half::Lower && self.r#powered == false { return 14269; }
        if self.r#facing == Facing::South && self.r#open == false && self.r#half == Half::Lower && self.r#hinge == Hinge::Right && self.r#powered == true { return 14272; }
        if self.r#hinge == Hinge::Left && self.r#half == Half::Lower && self.r#facing == Facing::West && self.r#open == false && self.r#powered == false { return 14285; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14250 {
            return Some(PaleOakDoor {
                r#open: true,
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14252 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 14282 {
            return Some(PaleOakDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 14281 {
            return Some(PaleOakDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: false,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 14280 {
            return Some(PaleOakDoor {
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 14270 {
            return Some(PaleOakDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Lower,
                r#open: true,
            });
        }
        if state_id == 14274 {
            return Some(PaleOakDoor {
                r#open: true,
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14257 {
            return Some(PaleOakDoor {
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 14283 {
            return Some(PaleOakDoor {
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 14260 {
            return Some(PaleOakDoor {
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 14293 {
            return Some(PaleOakDoor {
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 14264 {
            return Some(PaleOakDoor {
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 14296 {
            return Some(PaleOakDoor {
                r#facing: Facing::East,
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
            });
        }
        if state_id == 14301 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: false,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 14288 {
            return Some(PaleOakDoor {
                r#half: Half::Lower,
                r#open: false,
                r#powered: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14243 {
            return Some(PaleOakDoor {
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 14247 {
            return Some(PaleOakDoor {
                r#open: true,
                r#powered: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
            });
        }
        if state_id == 14271 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Right,
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 14277 {
            return Some(PaleOakDoor {
                r#facing: Facing::West,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#open: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 14275 {
            return Some(PaleOakDoor {
                r#half: Half::Upper,
                r#open: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 14290 {
            return Some(PaleOakDoor {
                r#powered: true,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 14256 {
            return Some(PaleOakDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 14266 {
            return Some(PaleOakDoor {
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 14248 {
            return Some(PaleOakDoor {
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
            });
        }
        if state_id == 14265 {
            return Some(PaleOakDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
            });
        }
        if state_id == 14286 {
            return Some(PaleOakDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#powered: true,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14297 {
            return Some(PaleOakDoor {
                r#open: false,
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 14300 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#powered: true,
                r#open: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 14303 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 14291 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Upper,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 14268 {
            return Some(PaleOakDoor {
                r#open: false,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#powered: true,
            });
        }
        if state_id == 14258 {
            return Some(PaleOakDoor {
                r#powered: true,
                r#half: Half::Upper,
                r#open: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14305 {
            return Some(PaleOakDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 14292 {
            return Some(PaleOakDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 14245 {
            return Some(PaleOakDoor {
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 14276 {
            return Some(PaleOakDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 14273 {
            return Some(PaleOakDoor {
                r#open: false,
                r#half: Half::Lower,
                r#powered: false,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14295 {
            return Some(PaleOakDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 14299 {
            return Some(PaleOakDoor {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#powered: false,
                r#open: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14253 {
            return Some(PaleOakDoor {
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 14259 {
            return Some(PaleOakDoor {
                r#powered: false,
                r#open: true,
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14304 {
            return Some(PaleOakDoor {
                r#open: false,
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14279 {
            return Some(PaleOakDoor {
                r#half: Half::Upper,
                r#powered: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 14242 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 14267 {
            return Some(PaleOakDoor {
                r#facing: Facing::South,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
            });
        }
        if state_id == 14244 {
            return Some(PaleOakDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 14254 {
            return Some(PaleOakDoor {
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::North,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14294 {
            return Some(PaleOakDoor {
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14287 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::West,
            });
        }
        if state_id == 14289 {
            return Some(PaleOakDoor {
                r#powered: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 14302 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 14246 {
            return Some(PaleOakDoor {
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 14262 {
            return Some(PaleOakDoor {
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 14263 {
            return Some(PaleOakDoor {
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: false,
            });
        }
        if state_id == 14278 {
            return Some(PaleOakDoor {
                r#powered: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 14284 {
            return Some(PaleOakDoor {
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14249 {
            return Some(PaleOakDoor {
                r#open: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#powered: false,
            });
        }
        if state_id == 14255 {
            return Some(PaleOakDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14298 {
            return Some(PaleOakDoor {
                r#half: Half::Lower,
                r#powered: true,
                r#open: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 14261 {
            return Some(PaleOakDoor {
                r#open: false,
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 14251 {
            return Some(PaleOakDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 14269 {
            return Some(PaleOakDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#open: false,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 14272 {
            return Some(PaleOakDoor {
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#powered: true,
            });
        }
        if state_id == 14285 {
            return Some(PaleOakDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
            });
        }
        return None;
    }
}

