use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleDoor {
    pub r#hinge: Hinge,
    pub powered: bool,
    pub r#facing: Facing,
    pub open: bool,
    pub r#half: Half,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hinge {
    Left,
    Right,
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

impl BlockState for JungleDoor {
    fn to_id(&self) -> i32 {
        if self.r#hinge == Hinge::Right && self.r#half == Half::Upper && self.r#open == true && self.r#powered == true && self.r#facing == Facing::West { return 14022; }
        if self.r#hinge == Hinge::Left && self.r#open == true && self.r#facing == Facing::West && self.r#half == Half::Lower && self.r#powered == true { return 14026; }
        if self.r#facing == Facing::East && self.r#open == true && self.r#powered == false && self.r#hinge == Hinge::Left && self.r#half == Half::Upper { return 14035; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#half == Half::Upper && self.r#hinge == Hinge::Right && self.r#open == false { return 14009; }
        if self.r#half == Half::Lower && self.r#hinge == Hinge::Right && self.r#facing == Facing::South && self.r#open == false && self.r#powered == false { return 14017; }
        if self.r#open == false && self.r#powered == true && self.r#facing == Facing::North && self.r#hinge == Hinge::Right && self.r#half == Half::Upper { return 13992; }
        if self.r#hinge == Hinge::Right && self.r#open == false && self.r#powered == true && self.r#half == Half::Lower && self.r#facing == Facing::West { return 14032; }
        if self.r#hinge == Hinge::Right && self.r#facing == Facing::East && self.r#open == true && self.r#powered == false && self.r#half == Half::Upper { return 14039; }
        if self.r#powered == false && self.r#open == false && self.r#facing == Facing::West && self.r#hinge == Hinge::Left && self.r#half == Half::Lower { return 14029; }
        if self.r#hinge == Hinge::Left && self.r#open == true && self.r#half == Half::Lower && self.r#powered == true && self.r#facing == Facing::South { return 14010; }
        if self.r#half == Half::Lower && self.r#open == true && self.r#powered == true && self.r#hinge == Hinge::Left && self.r#facing == Facing::East { return 14042; }
        if self.r#facing == Facing::East && self.r#half == Half::Upper && self.r#open == true && self.r#powered == true && self.r#hinge == Hinge::Right { return 14038; }
        if self.r#hinge == Hinge::Left && self.r#facing == Facing::South && self.r#half == Half::Upper && self.r#open == true && self.r#powered == true { return 14002; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#hinge == Hinge::Left && self.r#half == Half::Lower && self.r#open == false { return 14013; }
        if self.r#open == false && self.r#half == Half::Lower && self.r#facing == Facing::East && self.r#hinge == Hinge::Left && self.r#powered == true { return 14044; }
        if self.r#open == false && self.r#facing == Facing::South && self.r#powered == true && self.r#half == Half::Upper && self.r#hinge == Hinge::Left { return 14004; }
        if self.r#half == Half::Lower && self.r#hinge == Hinge::Right && self.r#facing == Facing::North && self.r#open == false && self.r#powered == true { return 14000; }
        if self.r#open == false && self.r#hinge == Hinge::Right && self.r#facing == Facing::North && self.r#powered == false && self.r#half == Half::Upper { return 13993; }
        if self.r#powered == false && self.r#half == Half::Upper && self.r#facing == Facing::South && self.r#open == false && self.r#hinge == Hinge::Left { return 14005; }
        if self.r#half == Half::Upper && self.r#facing == Facing::West && self.r#powered == true && self.r#open == false && self.r#hinge == Hinge::Right { return 14024; }
        if self.r#hinge == Hinge::Right && self.r#facing == Facing::West && self.r#powered == true && self.r#open == true && self.r#half == Half::Lower { return 14030; }
        if self.r#open == false && self.r#half == Half::Upper && self.r#powered == false && self.r#facing == Facing::East && self.r#hinge == Hinge::Left { return 14037; }
        if self.r#facing == Facing::East && self.r#open == false && self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#powered == false { return 14045; }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#open == false && self.r#half == Half::Upper && self.r#hinge == Hinge::Right { return 14008; }
        if self.r#open == true && self.r#powered == true && self.r#half == Half::Lower && self.r#facing == Facing::South && self.r#hinge == Hinge::Right { return 14014; }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#half == Half::Upper && self.r#hinge == Hinge::Right && self.r#open == false { return 14041; }
        if self.r#open == true && self.r#hinge == Hinge::Right && self.r#powered == false && self.r#facing == Facing::North && self.r#half == Half::Lower { return 13999; }
        if self.r#facing == Facing::South && self.r#half == Half::Lower && self.r#open == true && self.r#hinge == Hinge::Left && self.r#powered == false { return 14011; }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#open == false { return 14012; }
        if self.r#hinge == Hinge::Right && self.r#half == Half::Lower && self.r#open == true && self.r#powered == false && self.r#facing == Facing::West { return 14031; }
        if self.r#open == true && self.r#powered == true && self.r#half == Half::Upper && self.r#facing == Facing::North && self.r#hinge == Hinge::Right { return 13990; }
        if self.r#facing == Facing::North && self.r#half == Half::Lower && self.r#powered == false && self.r#hinge == Hinge::Left && self.r#open == true { return 13995; }
        if self.r#open == true && self.r#facing == Facing::West && self.r#hinge == Hinge::Left && self.r#half == Half::Upper && self.r#powered == true { return 14018; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#open == true && self.r#facing == Facing::North && self.r#powered == false { return 13987; }
        if self.r#facing == Facing::West && self.r#powered == false && self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#open == false { return 14021; }
        if self.r#facing == Facing::West && self.r#open == false && self.r#hinge == Hinge::Right && self.r#powered == false && self.r#half == Half::Upper { return 14025; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#open == false { return 14020; }
        if self.r#facing == Facing::North && self.r#hinge == Hinge::Right && self.r#open == true && self.r#powered == false && self.r#half == Half::Upper { return 13991; }
        if self.r#facing == Facing::East && self.r#hinge == Hinge::Left && self.r#half == Half::Upper && self.r#open == false && self.r#powered == true { return 14036; }
        if self.r#hinge == Hinge::Left && self.r#powered == false && self.r#half == Half::Lower && self.r#facing == Facing::North && self.r#open == false { return 13997; }
        if self.r#half == Half::Lower && self.r#powered == true && self.r#open == true && self.r#hinge == Hinge::Right && self.r#facing == Facing::East { return 14046; }
        if self.r#facing == Facing::North && self.r#half == Half::Lower && self.r#open == true && self.r#powered == true && self.r#hinge == Hinge::Left { return 13994; }
        if self.r#hinge == Hinge::Right && self.r#open == true && self.r#powered == true && self.r#half == Half::Lower && self.r#facing == Facing::North { return 13998; }
        if self.r#half == Half::Upper && self.r#powered == true && self.r#open == true && self.r#hinge == Hinge::Left && self.r#facing == Facing::North { return 13986; }
        if self.r#half == Half::Lower && self.r#facing == Facing::South && self.r#open == false && self.r#powered == true && self.r#hinge == Hinge::Right { return 14016; }
        if self.r#open == true && self.r#facing == Facing::East && self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#powered == false { return 14043; }
        if self.r#facing == Facing::South && self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#open == true && self.r#powered == false { return 14003; }
        if self.r#powered == true && self.r#facing == Facing::East && self.r#half == Half::Lower && self.r#hinge == Hinge::Right && self.r#open == false { return 14048; }
        if self.r#open == false && self.r#facing == Facing::North && self.r#powered == false && self.r#half == Half::Lower && self.r#hinge == Hinge::Right { return 14001; }
        if self.r#open == true && self.r#powered == true && self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#facing == Facing::East { return 14034; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Right && self.r#open == true && self.r#facing == Facing::South && self.r#powered == false { return 14007; }
        if self.r#open == true && self.r#hinge == Hinge::Right && self.r#half == Half::Lower && self.r#facing == Facing::East && self.r#powered == false { return 14047; }
        if self.r#facing == Facing::West && self.r#hinge == Hinge::Left && self.r#half == Half::Lower && self.r#open == false && self.r#powered == true { return 14028; }
        if self.r#facing == Facing::North && self.r#hinge == Hinge::Left && self.r#open == false && self.r#powered == false && self.r#half == Half::Upper { return 13989; }
        if self.r#hinge == Hinge::Right && self.r#powered == true && self.r#half == Half::Upper && self.r#facing == Facing::South && self.r#open == true { return 14006; }
        if self.r#half == Half::Upper && self.r#powered == false && self.r#facing == Facing::West && self.r#hinge == Hinge::Right && self.r#open == true { return 14023; }
        if self.r#facing == Facing::West && self.r#hinge == Hinge::Right && self.r#half == Half::Lower && self.r#powered == false && self.r#open == false { return 14033; }
        if self.r#facing == Facing::South && self.r#powered == false && self.r#half == Half::Lower && self.r#open == true && self.r#hinge == Hinge::Right { return 14015; }
        if self.r#hinge == Hinge::Left && self.r#half == Half::Upper && self.r#powered == true && self.r#facing == Facing::North && self.r#open == false { return 13988; }
        if self.r#facing == Facing::West && self.r#open == true && self.r#powered == false && self.r#half == Half::Upper && self.r#hinge == Hinge::Left { return 14019; }
        if self.r#facing == Facing::East && self.r#half == Half::Lower && self.r#hinge == Hinge::Right && self.r#powered == false && self.r#open == false { return 14049; }
        if self.r#half == Half::Upper && self.r#facing == Facing::East && self.r#hinge == Hinge::Right && self.r#open == false && self.r#powered == true { return 14040; }
        if self.r#open == false && self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#powered == true && self.r#facing == Facing::North { return 13996; }
        if self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#powered == false && self.r#open == true && self.r#facing == Facing::West { return 14027; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14022 {
            return Some(JungleDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 14026 {
            return Some(JungleDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#powered: true,
            });
        }
        if state_id == 14035 {
            return Some(JungleDoor {
                r#facing: Facing::East,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
            });
        }
        if state_id == 14009 {
            return Some(JungleDoor {
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 14017 {
            return Some(JungleDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 13992 {
            return Some(JungleDoor {
                r#open: false,
                r#powered: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
            });
        }
        if state_id == 14032 {
            return Some(JungleDoor {
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::West,
            });
        }
        if state_id == 14039 {
            return Some(JungleDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#open: true,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 14029 {
            return Some(JungleDoor {
                r#powered: false,
                r#open: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
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
        if state_id == 14042 {
            return Some(JungleDoor {
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 14038 {
            return Some(JungleDoor {
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14002 {
            return Some(JungleDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 14013 {
            return Some(JungleDoor {
                r#powered: false,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 14044 {
            return Some(JungleDoor {
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 14004 {
            return Some(JungleDoor {
                r#open: false,
                r#facing: Facing::South,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14000 {
            return Some(JungleDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 13993 {
            return Some(JungleDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 14005 {
            return Some(JungleDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14024 {
            return Some(JungleDoor {
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14030 {
            return Some(JungleDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#powered: true,
                r#open: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 14037 {
            return Some(JungleDoor {
                r#open: false,
                r#half: Half::Upper,
                r#powered: false,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14045 {
            return Some(JungleDoor {
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 14008 {
            return Some(JungleDoor {
                r#powered: true,
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14014 {
            return Some(JungleDoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14041 {
            return Some(JungleDoor {
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 13999 {
            return Some(JungleDoor {
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Lower,
            });
        }
        if state_id == 14011 {
            return Some(JungleDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 14012 {
            return Some(JungleDoor {
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 14031 {
            return Some(JungleDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: true,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 13990 {
            return Some(JungleDoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 13995 {
            return Some(JungleDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 14018 {
            return Some(JungleDoor {
                r#open: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 13987 {
            return Some(JungleDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 14021 {
            return Some(JungleDoor {
                r#facing: Facing::West,
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 14025 {
            return Some(JungleDoor {
                r#facing: Facing::West,
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 14020 {
            return Some(JungleDoor {
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 13991 {
            return Some(JungleDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 14036 {
            return Some(JungleDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 13997 {
            return Some(JungleDoor {
                r#hinge: Hinge::Left,
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 14046 {
            return Some(JungleDoor {
                r#half: Half::Lower,
                r#powered: true,
                r#open: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 13994 {
            return Some(JungleDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 13998 {
            return Some(JungleDoor {
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::North,
            });
        }
        if state_id == 13986 {
            return Some(JungleDoor {
                r#half: Half::Upper,
                r#powered: true,
                r#open: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 14016 {
            return Some(JungleDoor {
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14043 {
            return Some(JungleDoor {
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 14003 {
            return Some(JungleDoor {
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 14048 {
            return Some(JungleDoor {
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 14001 {
            return Some(JungleDoor {
                r#open: false,
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14034 {
            return Some(JungleDoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 14007 {
            return Some(JungleDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 14047 {
            return Some(JungleDoor {
                r#open: true,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 14028 {
            return Some(JungleDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 13989 {
            return Some(JungleDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 14006 {
            return Some(JungleDoor {
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 14023 {
            return Some(JungleDoor {
                r#half: Half::Upper,
                r#powered: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 14033 {
            return Some(JungleDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 14015 {
            return Some(JungleDoor {
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 13988 {
            return Some(JungleDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 14019 {
            return Some(JungleDoor {
                r#facing: Facing::West,
                r#open: true,
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14049 {
            return Some(JungleDoor {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 14040 {
            return Some(JungleDoor {
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 13996 {
            return Some(JungleDoor {
                r#open: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 14027 {
            return Some(JungleDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

