use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedCopperDoor {
    pub r#hinge: Hinge,
    pub powered: bool,
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

impl BlockState for WaxedCopperDoor {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West && self.r#open == true && self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#powered == true { return 26117; }
        if self.r#open == false && self.r#powered == false && self.r#facing == Facing::West && self.r#half == Half::Lower && self.r#hinge == Hinge::Left { return 26120; }
        if self.r#facing == Facing::East && self.r#hinge == Hinge::Right && self.r#open == false && self.r#powered == true && self.r#half == Half::Lower { return 26139; }
        if self.r#open == false && self.r#powered == false && self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#facing == Facing::East { return 26136; }
        if self.r#facing == Facing::West && self.r#half == Half::Upper && self.r#powered == false && self.r#open == true && self.r#hinge == Hinge::Right { return 26114; }
        if self.r#facing == Facing::East && self.r#powered == true && self.r#hinge == Hinge::Left && self.r#open == false && self.r#half == Half::Lower { return 26135; }
        if self.r#hinge == Hinge::Right && self.r#open == true && self.r#powered == true && self.r#half == Half::Upper && self.r#facing == Facing::South { return 26097; }
        if self.r#hinge == Hinge::Right && self.r#open == true && self.r#powered == true && self.r#facing == Facing::North && self.r#half == Half::Lower { return 26089; }
        if self.r#facing == Facing::South && self.r#hinge == Hinge::Left && self.r#open == true && self.r#powered == true && self.r#half == Half::Upper { return 26093; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#open == false && self.r#powered == false && self.r#facing == Facing::East { return 26128; }
        if self.r#half == Half::Lower && self.r#open == false && self.r#facing == Facing::West && self.r#powered == false && self.r#hinge == Hinge::Right { return 26124; }
        if self.r#facing == Facing::North && self.r#powered == false && self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#open == false { return 26088; }
        if self.r#facing == Facing::West && self.r#hinge == Hinge::Left && self.r#powered == true && self.r#half == Half::Upper && self.r#open == true { return 26109; }
        if self.r#open == false && self.r#hinge == Hinge::Left && self.r#powered == false && self.r#half == Half::Upper && self.r#facing == Facing::West { return 26112; }
        if self.r#hinge == Hinge::Right && self.r#half == Half::Upper && self.r#facing == Facing::North && self.r#open == true && self.r#powered == false { return 26082; }
        if self.r#facing == Facing::East && self.r#open == true && self.r#half == Half::Lower && self.r#powered == true && self.r#hinge == Hinge::Right { return 26137; }
        if self.r#hinge == Hinge::Left && self.r#open == true && self.r#half == Half::Lower && self.r#facing == Facing::South && self.r#powered == false { return 26102; }
        if self.r#powered == true && self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#facing == Facing::South && self.r#open == true { return 26101; }
        if self.r#open == false && self.r#powered == true && self.r#hinge == Hinge::Right && self.r#facing == Facing::East && self.r#half == Half::Upper { return 26131; }
        if self.r#half == Half::Lower && self.r#facing == Facing::West && self.r#hinge == Hinge::Left && self.r#open == true && self.r#powered == false { return 26118; }
        if self.r#half == Half::Lower && self.r#open == true && self.r#powered == true && self.r#facing == Facing::North && self.r#hinge == Hinge::Left { return 26085; }
        if self.r#facing == Facing::North && self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#powered == false && self.r#open == true { return 26086; }
        if self.r#hinge == Hinge::Left && self.r#facing == Facing::North && self.r#half == Half::Upper && self.r#open == false && self.r#powered == false { return 26080; }
        if self.r#half == Half::Upper && self.r#open == false && self.r#powered == false && self.r#hinge == Hinge::Right && self.r#facing == Facing::West { return 26116; }
        if self.r#powered == true && self.r#open == true && self.r#half == Half::Upper && self.r#facing == Facing::West && self.r#hinge == Hinge::Right { return 26113; }
        if self.r#half == Half::Upper && self.r#facing == Facing::East && self.r#powered == false && self.r#open == true && self.r#hinge == Hinge::Right { return 26130; }
        if self.r#hinge == Hinge::Right && self.r#powered == false && self.r#half == Half::Upper && self.r#open == false && self.r#facing == Facing::North { return 26084; }
        if self.r#hinge == Hinge::Left && self.r#half == Half::Upper && self.r#powered == true && self.r#facing == Facing::South && self.r#open == false { return 26095; }
        if self.r#powered == true && self.r#half == Half::Upper && self.r#facing == Facing::West && self.r#open == false && self.r#hinge == Hinge::Right { return 26115; }
        if self.r#open == true && self.r#facing == Facing::South && self.r#half == Half::Upper && self.r#powered == false && self.r#hinge == Hinge::Right { return 26098; }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#half == Half::Lower && self.r#open == false && self.r#hinge == Hinge::Right { return 26107; }
        if self.r#open == true && self.r#half == Half::Upper && self.r#facing == Facing::East && self.r#hinge == Hinge::Right && self.r#powered == true { return 26129; }
        if self.r#half == Half::Upper && self.r#powered == true && self.r#facing == Facing::East && self.r#hinge == Hinge::Left && self.r#open == false { return 26127; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#open == false && self.r#hinge == Hinge::Right && self.r#half == Half::Lower { return 26108; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#powered == false && self.r#open == true && self.r#facing == Facing::North { return 26078; }
        if self.r#powered == false && self.r#open == false && self.r#facing == Facing::North && self.r#half == Half::Lower && self.r#hinge == Hinge::Right { return 26092; }
        if self.r#open == false && self.r#powered == false && self.r#facing == Facing::South && self.r#half == Half::Upper && self.r#hinge == Hinge::Left { return 26096; }
        if self.r#hinge == Hinge::Right && self.r#half == Half::Lower && self.r#powered == false && self.r#open == true && self.r#facing == Facing::East { return 26138; }
        if self.r#half == Half::Lower && self.r#powered == false && self.r#open == true && self.r#facing == Facing::West && self.r#hinge == Hinge::Right { return 26122; }
        if self.r#open == false && self.r#hinge == Hinge::Right && self.r#facing == Facing::North && self.r#half == Half::Lower && self.r#powered == true { return 26091; }
        if self.r#powered == false && self.r#hinge == Hinge::Right && self.r#facing == Facing::North && self.r#half == Half::Lower && self.r#open == true { return 26090; }
        if self.r#open == false && self.r#half == Half::Upper && self.r#powered == true && self.r#facing == Facing::South && self.r#hinge == Hinge::Right { return 26099; }
        if self.r#open == true && self.r#powered == false && self.r#facing == Facing::West && self.r#half == Half::Upper && self.r#hinge == Hinge::Left { return 26110; }
        if self.r#powered == true && self.r#open == false && self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#facing == Facing::West { return 26119; }
        if self.r#open == true && self.r#powered == true && self.r#facing == Facing::North && self.r#hinge == Hinge::Right && self.r#half == Half::Upper { return 26081; }
        if self.r#open == false && self.r#powered == true && self.r#facing == Facing::West && self.r#hinge == Hinge::Right && self.r#half == Half::Lower { return 26123; }
        if self.r#open == false && self.r#powered == false && self.r#hinge == Hinge::Right && self.r#facing == Facing::East && self.r#half == Half::Lower { return 26140; }
        if self.r#facing == Facing::East && self.r#half == Half::Upper && self.r#open == false && self.r#hinge == Hinge::Right && self.r#powered == false { return 26132; }
        if self.r#half == Half::Lower && self.r#powered == true && self.r#facing == Facing::West && self.r#hinge == Hinge::Right && self.r#open == true { return 26121; }
        if self.r#facing == Facing::South && self.r#hinge == Hinge::Left && self.r#open == false && self.r#half == Half::Lower && self.r#powered == false { return 26104; }
        if self.r#half == Half::Upper && self.r#powered == true && self.r#hinge == Hinge::Left && self.r#open == true && self.r#facing == Facing::East { return 26125; }
        if self.r#powered == false && self.r#hinge == Hinge::Left && self.r#half == Half::Lower && self.r#facing == Facing::East && self.r#open == true { return 26134; }
        if self.r#powered == true && self.r#open == true && self.r#facing == Facing::South && self.r#half == Half::Lower && self.r#hinge == Hinge::Right { return 26105; }
        if self.r#hinge == Hinge::Left && self.r#half == Half::Upper && self.r#facing == Facing::North && self.r#open == false && self.r#powered == true { return 26079; }
        if self.r#facing == Facing::South && self.r#hinge == Hinge::Left && self.r#open == false && self.r#powered == true && self.r#half == Half::Lower { return 26103; }
        if self.r#hinge == Hinge::Left && self.r#facing == Facing::North && self.r#half == Half::Upper && self.r#open == true && self.r#powered == true { return 26077; }
        if self.r#powered == false && self.r#half == Half::Upper && self.r#facing == Facing::East && self.r#hinge == Hinge::Left && self.r#open == true { return 26126; }
        if self.r#facing == Facing::North && self.r#hinge == Hinge::Right && self.r#open == false && self.r#powered == true && self.r#half == Half::Upper { return 26083; }
        if self.r#powered == false && self.r#half == Half::Upper && self.r#facing == Facing::South && self.r#hinge == Hinge::Right && self.r#open == false { return 26100; }
        if self.r#facing == Facing::West && self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#open == false && self.r#powered == true { return 26111; }
        if self.r#open == true && self.r#facing == Facing::East && self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#powered == true { return 26133; }
        if self.r#open == true && self.r#facing == Facing::South && self.r#hinge == Hinge::Left && self.r#powered == false && self.r#half == Half::Upper { return 26094; }
        if self.r#half == Half::Lower && self.r#facing == Facing::North && self.r#open == false && self.r#powered == true && self.r#hinge == Hinge::Left { return 26087; }
        if self.r#open == true && self.r#facing == Facing::South && self.r#hinge == Hinge::Right && self.r#powered == false && self.r#half == Half::Lower { return 26106; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26117 {
            return Some(WaxedCopperDoor {
                r#facing: Facing::West,
                r#open: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 26120 {
            return Some(WaxedCopperDoor {
                r#open: false,
                r#powered: false,
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26139 {
            return Some(WaxedCopperDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 26136 {
            return Some(WaxedCopperDoor {
                r#open: false,
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 26114 {
            return Some(WaxedCopperDoor {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#powered: false,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26135 {
            return Some(WaxedCopperDoor {
                r#facing: Facing::East,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#open: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 26097 {
            return Some(WaxedCopperDoor {
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::South,
            });
        }
        if state_id == 26089 {
            return Some(WaxedCopperDoor {
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Lower,
            });
        }
        if state_id == 26093 {
            return Some(WaxedCopperDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 26128 {
            return Some(WaxedCopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 26124 {
            return Some(WaxedCopperDoor {
                r#half: Half::Lower,
                r#open: false,
                r#facing: Facing::West,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26088 {
            return Some(WaxedCopperDoor {
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 26109 {
            return Some(WaxedCopperDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 26112 {
            return Some(WaxedCopperDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::West,
            });
        }
        if state_id == 26082 {
            return Some(WaxedCopperDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 26137 {
            return Some(WaxedCopperDoor {
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Lower,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26102 {
            return Some(WaxedCopperDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 26101 {
            return Some(WaxedCopperDoor {
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 26131 {
            return Some(WaxedCopperDoor {
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#half: Half::Upper,
            });
        }
        if state_id == 26118 {
            return Some(WaxedCopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 26085 {
            return Some(WaxedCopperDoor {
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26086 {
            return Some(WaxedCopperDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 26080 {
            return Some(WaxedCopperDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26116 {
            return Some(WaxedCopperDoor {
                r#half: Half::Upper,
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
            });
        }
        if state_id == 26113 {
            return Some(WaxedCopperDoor {
                r#powered: true,
                r#open: true,
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26130 {
            return Some(WaxedCopperDoor {
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#powered: false,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26084 {
            return Some(WaxedCopperDoor {
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 26095 {
            return Some(WaxedCopperDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 26115 {
            return Some(WaxedCopperDoor {
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#open: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26098 {
            return Some(WaxedCopperDoor {
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26107 {
            return Some(WaxedCopperDoor {
                r#facing: Facing::South,
                r#powered: true,
                r#half: Half::Lower,
                r#open: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26129 {
            return Some(WaxedCopperDoor {
                r#open: true,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#powered: true,
            });
        }
        if state_id == 26127 {
            return Some(WaxedCopperDoor {
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 26108 {
            return Some(WaxedCopperDoor {
                r#powered: false,
                r#facing: Facing::South,
                r#open: false,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
            });
        }
        if state_id == 26078 {
            return Some(WaxedCopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26092 {
            return Some(WaxedCopperDoor {
                r#powered: false,
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26096 {
            return Some(WaxedCopperDoor {
                r#open: false,
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26138 {
            return Some(WaxedCopperDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#powered: false,
                r#open: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 26122 {
            return Some(WaxedCopperDoor {
                r#half: Half::Lower,
                r#powered: false,
                r#open: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26091 {
            return Some(WaxedCopperDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#powered: true,
            });
        }
        if state_id == 26090 {
            return Some(WaxedCopperDoor {
                r#powered: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: true,
            });
        }
        if state_id == 26099 {
            return Some(WaxedCopperDoor {
                r#open: false,
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26110 {
            return Some(WaxedCopperDoor {
                r#open: true,
                r#powered: false,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26119 {
            return Some(WaxedCopperDoor {
                r#powered: true,
                r#open: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
            });
        }
        if state_id == 26081 {
            return Some(WaxedCopperDoor {
                r#open: true,
                r#powered: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
            });
        }
        if state_id == 26123 {
            return Some(WaxedCopperDoor {
                r#open: false,
                r#powered: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
            });
        }
        if state_id == 26140 {
            return Some(WaxedCopperDoor {
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#half: Half::Lower,
            });
        }
        if state_id == 26132 {
            return Some(WaxedCopperDoor {
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: false,
            });
        }
        if state_id == 26121 {
            return Some(WaxedCopperDoor {
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 26104 {
            return Some(WaxedCopperDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#open: false,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 26125 {
            return Some(WaxedCopperDoor {
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#open: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 26134 {
            return Some(WaxedCopperDoor {
                r#powered: false,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 26105 {
            return Some(WaxedCopperDoor {
                r#powered: true,
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26079 {
            return Some(WaxedCopperDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26103 {
            return Some(WaxedCopperDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 26077 {
            return Some(WaxedCopperDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26126 {
            return Some(WaxedCopperDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 26083 {
            return Some(WaxedCopperDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 26100 {
            return Some(WaxedCopperDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 26111 {
            return Some(WaxedCopperDoor {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26133 {
            return Some(WaxedCopperDoor {
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 26094 {
            return Some(WaxedCopperDoor {
                r#open: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 26087 {
            return Some(WaxedCopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26106 {
            return Some(WaxedCopperDoor {
                r#open: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Lower,
            });
        }
        return None;
    }
}

