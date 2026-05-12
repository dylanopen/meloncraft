use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleDoor {
    pub powered: bool,
    pub r#half: Half,
    pub r#facing: Facing,
    pub r#hinge: Hinge,
    pub open: bool,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hinge {
    Left,
    Right,
}

impl BlockState for JungleDoor {
    fn to_id(self) -> i32 {
        if block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#half == Half::Lower { return 13996; }
        if block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#open == false { return 14017; }
        if block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::North && block_state.r#powered == true { return 14000; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#open == false { return 14001; }
        if block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#hinge == Hinge::Left { return 14005; }
        if block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#powered == false { return 13999; }
        if block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#open == false { return 14004; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Lower { return 14043; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == true { return 14028; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#half == Half::Upper { return 14019; }
        if block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right { return 14006; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#open == true { return 14034; }
        if block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right { return 14041; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right { return 14014; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#open == false { return 14044; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#powered == false { return 14021; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#facing == Facing::East { return 14048; }
        if block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left && block_state.r#open == true { return 14035; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#facing == Facing::South { return 14010; }
        if block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#open == true { return 14039; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::East { return 14045; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == true && block_state.r#hinge == Hinge::Left { return 13988; }
        if block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == false && block_state.r#half == Half::Lower { return 13997; }
        if block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left { return 14026; }
        if block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#half == Half::Lower { return 14031; }
        if block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::North && block_state.r#open == true { return 13991; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::South { return 14015; }
        if block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Lower { return 14032; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#open == false { return 14040; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#powered == true && block_state.r#hinge == Hinge::Right { return 13990; }
        if block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#open == true { return 13987; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper { return 14003; }
        if block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::West && block_state.r#open == true { return 14022; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::East { return 14046; }
        if block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#hinge == Hinge::Left { return 13986; }
        if block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#facing == Facing::North { return 13992; }
        if block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#powered == true { return 14008; }
        if block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#powered == true { return 14012; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left { return 14042; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == false { return 14011; }
        if block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left && block_state.r#open == false { return 14029; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#powered == true { return 14030; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::East { return 14038; }
        if block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper { return 14007; }
        if block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#powered == true { return 14002; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower { return 14033; }
        if block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#powered == false { return 14047; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper { return 14009; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == true { return 14020; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::West { return 14027; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == true { return 13998; }
        if block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#hinge == Hinge::Left { return 14037; }
        if block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Left { return 14013; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#open == true { return 13994; }
        if block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#facing == Facing::West { return 14025; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == true { return 14036; }
        if block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#powered == false { return 13995; }
        if block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right { return 14049; }
        if block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#facing == Facing::West && block_state.r#open == true { return 14023; }
        if block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#facing == Facing::West { return 14018; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#half == Half::Lower { return 14016; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::West { return 14024; }
        if block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#powered == false { return 13989; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#half == Half::Upper { return 13993; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13996 {
            return Some(JungleDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 14017 {
            return Some(JungleDoor {
                r#half: Half::Lower,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 14000 {
            return Some(JungleDoor {
                r#open: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 14001 {
            return Some(JungleDoor {
                r#powered: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 14005 {
            return Some(JungleDoor {
                r#half: Half::Upper,
                r#powered: false,
                r#facing: Facing::South,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 13999 {
            return Some(JungleDoor {
                r#hinge: Hinge::Right,
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 14004 {
            return Some(JungleDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 14043 {
            return Some(JungleDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Lower,
            });
        }
        if state_id == 14028 {
            return Some(JungleDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 14019 {
            return Some(JungleDoor {
                r#powered: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 14006 {
            return Some(JungleDoor {
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14034 {
            return Some(JungleDoor {
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 14041 {
            return Some(JungleDoor {
                r#open: false,
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14014 {
            return Some(JungleDoor {
                r#open: true,
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14044 {
            return Some(JungleDoor {
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 14021 {
            return Some(JungleDoor {
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#open: false,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 14048 {
            return Some(JungleDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 14035 {
            return Some(JungleDoor {
                r#half: Half::Upper,
                r#powered: false,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 14010 {
            return Some(JungleDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 14039 {
            return Some(JungleDoor {
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 14045 {
            return Some(JungleDoor {
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::East,
            });
        }
        if state_id == 13988 {
            return Some(JungleDoor {
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 13997 {
            return Some(JungleDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#open: false,
                r#powered: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 14026 {
            return Some(JungleDoor {
                r#facing: Facing::West,
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14031 {
            return Some(JungleDoor {
                r#facing: Facing::West,
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 13991 {
            return Some(JungleDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 14015 {
            return Some(JungleDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 14032 {
            return Some(JungleDoor {
                r#hinge: Hinge::Right,
                r#powered: true,
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Lower,
            });
        }
        if state_id == 14040 {
            return Some(JungleDoor {
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 13990 {
            return Some(JungleDoor {
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 13987 {
            return Some(JungleDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 14003 {
            return Some(JungleDoor {
                r#facing: Facing::South,
                r#powered: false,
                r#open: true,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
            });
        }
        if state_id == 14022 {
            return Some(JungleDoor {
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 14046 {
            return Some(JungleDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#powered: true,
                r#open: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 13986 {
            return Some(JungleDoor {
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::North,
                r#open: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 13992 {
            return Some(JungleDoor {
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 14008 {
            return Some(JungleDoor {
                r#open: false,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 14012 {
            return Some(JungleDoor {
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 14042 {
            return Some(JungleDoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14011 {
            return Some(JungleDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 14029 {
            return Some(JungleDoor {
                r#half: Half::Lower,
                r#powered: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 14030 {
            return Some(JungleDoor {
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: true,
            });
        }
        if state_id == 14038 {
            return Some(JungleDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 14007 {
            return Some(JungleDoor {
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Upper,
            });
        }
        if state_id == 14002 {
            return Some(JungleDoor {
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 14033 {
            return Some(JungleDoor {
                r#powered: false,
                r#open: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
            });
        }
        if state_id == 14047 {
            return Some(JungleDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 14009 {
            return Some(JungleDoor {
                r#powered: false,
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#half: Half::Upper,
            });
        }
        if state_id == 14020 {
            return Some(JungleDoor {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 14027 {
            return Some(JungleDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::West,
            });
        }
        if state_id == 13998 {
            return Some(JungleDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 14037 {
            return Some(JungleDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14013 {
            return Some(JungleDoor {
                r#half: Half::Lower,
                r#powered: false,
                r#open: false,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 13994 {
            return Some(JungleDoor {
                r#powered: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: true,
            });
        }
        if state_id == 14025 {
            return Some(JungleDoor {
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 14036 {
            return Some(JungleDoor {
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 13995 {
            return Some(JungleDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 14049 {
            return Some(JungleDoor {
                r#facing: Facing::East,
                r#open: false,
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14023 {
            return Some(JungleDoor {
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 14018 {
            return Some(JungleDoor {
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 14016 {
            return Some(JungleDoor {
                r#facing: Facing::South,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 14024 {
            return Some(JungleDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 13989 {
            return Some(JungleDoor {
                r#open: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 13993 {
            return Some(JungleDoor {
                r#facing: Facing::North,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Upper,
            });
        }
        return None;
    }
}

