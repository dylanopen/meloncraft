use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaDoor {
    pub r#hinge: Hinge,
    pub powered: bool,
    pub r#facing: Facing,
    pub r#half: Half,
    pub open: bool,
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

impl BlockState for AcaciaDoor {
    fn to_id(&self) -> i32 {
        if self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#open == true
        {
            return 14079;
        }
        if self.r#hinge == Hinge::Left
            && self.r#half == Half::Lower
            && self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#open == true
        {
            return 14090;
        }
        if self.r#hinge == Hinge::Left
            && self.r#facing == Facing::South
            && self.r#half == Half::Upper
            && self.r#open == false
            && self.r#powered == false
        {
            return 14069;
        }
        if self.r#open == true
            && self.r#hinge == Hinge::Right
            && self.r#powered == false
            && self.r#half == Half::Upper
            && self.r#facing == Facing::West
        {
            return 14087;
        }
        if self.r#powered == false
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Upper
            && self.r#facing == Facing::North
            && self.r#open == true
        {
            return 14051;
        }
        if self.r#open == false
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#half == Half::Lower
        {
            return 14064;
        }
        if self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#half == Half::Upper
            && self.r#open == false
            && self.r#hinge == Hinge::Right
        {
            return 14089;
        }
        if self.r#half == Half::Lower
            && self.r#open == true
            && self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#hinge == Hinge::Right
        {
            return 14094;
        }
        if self.r#powered == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
            && self.r#open == true
        {
            return 14095;
        }
        if self.r#half == Half::Lower
            && self.r#open == false
            && self.r#powered == false
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::West
        {
            return 14097;
        }
        if self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Left
            && self.r#open == true
        {
            return 14106;
        }
        if self.r#hinge == Hinge::Right
            && self.r#half == Half::Upper
            && self.r#open == true
            && self.r#powered == false
            && self.r#facing == Facing::South
        {
            return 14071;
        }
        if self.r#hinge == Hinge::Left
            && self.r#half == Half::Lower
            && self.r#powered == true
            && self.r#facing == Facing::North
            && self.r#open == true
        {
            return 14058;
        }
        if self.r#half == Half::Upper
            && self.r#facing == Facing::South
            && self.r#powered == true
            && self.r#open == false
            && self.r#hinge == Hinge::Left
        {
            return 14068;
        }
        if self.r#hinge == Hinge::Left
            && self.r#open == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Upper
            && self.r#powered == true
        {
            return 14084;
        }
        if self.r#powered == false
            && self.r#facing == Facing::West
            && self.r#open == true
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Upper
        {
            return 14083;
        }
        if self.r#open == true
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::South
            && self.r#half == Half::Lower
            && self.r#powered == false
        {
            return 14075;
        }
        if self.r#facing == Facing::West
            && self.r#open == false
            && self.r#half == Half::Lower
            && self.r#powered == false
            && self.r#hinge == Hinge::Left
        {
            return 14093;
        }
        if self.r#half == Half::Upper
            && self.r#hinge == Hinge::Left
            && self.r#open == true
            && self.r#powered == true
            && self.r#facing == Facing::North
        {
            return 14050;
        }
        if self.r#open == true
            && self.r#powered == true
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::West
            && self.r#half == Half::Upper
        {
            return 14082;
        }
        if self.r#hinge == Hinge::Left
            && self.r#half == Half::Upper
            && self.r#open == false
            && self.r#facing == Facing::East
            && self.r#powered == true
        {
            return 14100;
        }
        if self.r#open == false
            && self.r#facing == Facing::East
            && self.r#hinge == Hinge::Left
            && self.r#powered == false
            && self.r#half == Half::Upper
        {
            return 14101;
        }
        if self.r#open == false
            && self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Upper
        {
            return 14105;
        }
        if self.r#hinge == Hinge::Left
            && self.r#facing == Facing::East
            && self.r#half == Half::Lower
            && self.r#open == false
            && self.r#powered == false
        {
            return 14109;
        }
        if self.r#powered == true
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::North
            && self.r#open == true
        {
            return 14054;
        }
        if self.r#hinge == Hinge::Left
            && self.r#half == Half::Upper
            && self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#open == true
        {
            return 14066;
        }
        if self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
            && self.r#open == false
            && self.r#facing == Facing::North
            && self.r#powered == true
        {
            return 14056;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Left
            && self.r#open == false
            && self.r#powered == false
        {
            return 14053;
        }
        if self.r#powered == false
            && self.r#half == Half::Upper
            && self.r#open == false
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::North
        {
            return 14057;
        }
        if self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
            && self.r#powered == false
            && self.r#open == true
            && self.r#facing == Facing::North
        {
            return 14063;
        }
        if self.r#half == Half::Lower
            && self.r#open == false
            && self.r#hinge == Hinge::Right
            && self.r#powered == true
            && self.r#facing == Facing::South
        {
            return 14080;
        }
        if self.r#half == Half::Upper
            && self.r#powered == false
            && self.r#facing == Facing::West
            && self.r#open == false
            && self.r#hinge == Hinge::Left
        {
            return 14085;
        }
        if self.r#hinge == Hinge::Left
            && self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#open == false
            && self.r#half == Half::Lower
        {
            return 14076;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Lower
            && self.r#open == true
            && self.r#powered == true
            && self.r#hinge == Hinge::Right
        {
            return 14110;
        }
        if self.r#open == true
            && self.r#powered == false
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::East
            && self.r#half == Half::Upper
        {
            return 14099;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Lower
            && self.r#powered == true
            && self.r#hinge == Hinge::Right
            && self.r#open == false
        {
            return 14112;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Lower
            && self.r#open == false
            && self.r#powered == true
            && self.r#hinge == Hinge::Left
        {
            return 14060;
        }
        if self.r#hinge == Hinge::Right
            && self.r#half == Half::Lower
            && self.r#open == false
            && self.r#facing == Facing::South
            && self.r#powered == false
        {
            return 14081;
        }
        if self.r#half == Half::Lower
            && self.r#open == false
            && self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#hinge == Hinge::Left
        {
            return 14092;
        }
        if self.r#half == Half::Upper
            && self.r#open == false
            && self.r#facing == Facing::West
            && self.r#hinge == Hinge::Right
            && self.r#powered == true
        {
            return 14088;
        }
        if self.r#powered == false
            && self.r#half == Half::Lower
            && self.r#open == true
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::North
        {
            return 14059;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Upper
            && self.r#powered == true
            && self.r#hinge == Hinge::Right
            && self.r#open == false
        {
            return 14072;
        }
        if self.r#half == Half::Upper
            && self.r#hinge == Hinge::Left
            && self.r#open == false
            && self.r#powered == true
            && self.r#facing == Facing::North
        {
            return 14052;
        }
        if self.r#powered == false
            && self.r#facing == Facing::East
            && self.r#half == Half::Lower
            && self.r#open == false
            && self.r#hinge == Hinge::Right
        {
            return 14113;
        }
        if self.r#hinge == Hinge::Left
            && self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#half == Half::Lower
            && self.r#open == false
        {
            return 14077;
        }
        if self.r#open == false
            && self.r#powered == true
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::East
            && self.r#half == Half::Lower
        {
            return 14108;
        }
        if self.r#hinge == Hinge::Right
            && self.r#facing == Facing::North
            && self.r#half == Half::Lower
            && self.r#open == false
            && self.r#powered == false
        {
            return 14065;
        }
        if self.r#hinge == Hinge::Left
            && self.r#half == Half::Upper
            && self.r#open == true
            && self.r#facing == Facing::South
            && self.r#powered == false
        {
            return 14067;
        }
        if self.r#powered == false
            && self.r#open == false
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::North
        {
            return 14061;
        }
        if self.r#powered == true
            && self.r#half == Half::Upper
            && self.r#open == true
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::West
        {
            return 14086;
        }
        if self.r#open == true
            && self.r#half == Half::Upper
            && self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#hinge == Hinge::Right
        {
            return 14070;
        }
        if self.r#powered == false
            && self.r#half == Half::Upper
            && self.r#facing == Facing::North
            && self.r#hinge == Hinge::Right
            && self.r#open == true
        {
            return 14055;
        }
        if self.r#hinge == Hinge::Right
            && self.r#facing == Facing::North
            && self.r#half == Half::Lower
            && self.r#open == true
            && self.r#powered == true
        {
            return 14062;
        }
        if self.r#half == Half::Lower
            && self.r#open == true
            && self.r#facing == Facing::South
            && self.r#powered == true
            && self.r#hinge == Hinge::Left
        {
            return 14074;
        }
        if self.r#powered == true
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Upper
            && self.r#open == true
            && self.r#facing == Facing::East
        {
            return 14102;
        }
        if self.r#powered == false
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Upper
            && self.r#open == false
            && self.r#facing == Facing::South
        {
            return 14073;
        }
        if self.r#half == Half::Lower
            && self.r#open == true
            && self.r#powered == false
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::West
        {
            return 14091;
        }
        if self.r#half == Half::Lower
            && self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#hinge == Hinge::Left
            && self.r#open == true
        {
            return 14107;
        }
        if self.r#half == Half::Lower
            && self.r#powered == true
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::South
            && self.r#open == true
        {
            return 14078;
        }
        if self.r#open == false
            && self.r#half == Half::Lower
            && self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#hinge == Hinge::Right
        {
            return 14096;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
            && self.r#powered == false
            && self.r#open == true
        {
            return 14103;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Lower
            && self.r#open == true
            && self.r#powered == false
            && self.r#hinge == Hinge::Right
        {
            return 14111;
        }
        if self.r#half == Half::Upper
            && self.r#open == true
            && self.r#powered == true
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::East
        {
            return 14098;
        }
        if self.r#hinge == Hinge::Right
            && self.r#facing == Facing::East
            && self.r#open == false
            && self.r#half == Half::Upper
            && self.r#powered == true
        {
            return 14104;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14079 {
            return Some(AcaciaDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 14090 {
            return Some(AcaciaDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 14069 {
            return Some(AcaciaDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 14087 {
            return Some(AcaciaDoor {
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::West,
            });
        }
        if state_id == 14051 {
            return Some(AcaciaDoor {
                r#powered: false,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 14064 {
            return Some(AcaciaDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 14089 {
            return Some(AcaciaDoor {
                r#facing: Facing::West,
                r#powered: false,
                r#half: Half::Upper,
                r#open: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14094 {
            return Some(AcaciaDoor {
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14095 {
            return Some(AcaciaDoor {
                r#powered: false,
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 14097 {
            return Some(AcaciaDoor {
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
            });
        }
        if state_id == 14106 {
            return Some(AcaciaDoor {
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 14071 {
            return Some(AcaciaDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#open: true,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 14058 {
            return Some(AcaciaDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 14068 {
            return Some(AcaciaDoor {
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14084 {
            return Some(AcaciaDoor {
                r#hinge: Hinge::Left,
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 14083 {
            return Some(AcaciaDoor {
                r#powered: false,
                r#facing: Facing::West,
                r#open: true,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
            });
        }
        if state_id == 14075 {
            return Some(AcaciaDoor {
                r#open: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 14093 {
            return Some(AcaciaDoor {
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Lower,
                r#powered: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14050 {
            return Some(AcaciaDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 14082 {
            return Some(AcaciaDoor {
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#half: Half::Upper,
            });
        }
        if state_id == 14100 {
            return Some(AcaciaDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 14101 {
            return Some(AcaciaDoor {
                r#open: false,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 14105 {
            return Some(AcaciaDoor {
                r#open: false,
                r#facing: Facing::East,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
            });
        }
        if state_id == 14109 {
            return Some(AcaciaDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 14054 {
            return Some(AcaciaDoor {
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 14066 {
            return Some(AcaciaDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 14056 {
            return Some(AcaciaDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: false,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 14053 {
            return Some(AcaciaDoor {
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 14057 {
            return Some(AcaciaDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 14063 {
            return Some(AcaciaDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#open: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 14080 {
            return Some(AcaciaDoor {
                r#half: Half::Lower,
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 14085 {
            return Some(AcaciaDoor {
                r#half: Half::Upper,
                r#powered: false,
                r#facing: Facing::West,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14076 {
            return Some(AcaciaDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 14110 {
            return Some(AcaciaDoor {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14099 {
            return Some(AcaciaDoor {
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#half: Half::Upper,
            });
        }
        if state_id == 14112 {
            return Some(AcaciaDoor {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 14060 {
            return Some(AcaciaDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14081 {
            return Some(AcaciaDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: false,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 14092 {
            return Some(AcaciaDoor {
                r#half: Half::Lower,
                r#open: false,
                r#powered: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14088 {
            return Some(AcaciaDoor {
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#powered: true,
            });
        }
        if state_id == 14059 {
            return Some(AcaciaDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 14072 {
            return Some(AcaciaDoor {
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 14052 {
            return Some(AcaciaDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 14113 {
            return Some(AcaciaDoor {
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#open: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14077 {
            return Some(AcaciaDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 14108 {
            return Some(AcaciaDoor {
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#half: Half::Lower,
            });
        }
        if state_id == 14065 {
            return Some(AcaciaDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 14067 {
            return Some(AcaciaDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 14061 {
            return Some(AcaciaDoor {
                r#powered: false,
                r#open: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 14086 {
            return Some(AcaciaDoor {
                r#powered: true,
                r#half: Half::Upper,
                r#open: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
            });
        }
        if state_id == 14070 {
            return Some(AcaciaDoor {
                r#open: true,
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14055 {
            return Some(AcaciaDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 14062 {
            return Some(AcaciaDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 14074 {
            return Some(AcaciaDoor {
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::South,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14102 {
            return Some(AcaciaDoor {
                r#powered: true,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#open: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 14073 {
            return Some(AcaciaDoor {
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 14091 {
            return Some(AcaciaDoor {
                r#half: Half::Lower,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
            });
        }
        if state_id == 14107 {
            return Some(AcaciaDoor {
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 14078 {
            return Some(AcaciaDoor {
                r#half: Half::Lower,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 14096 {
            return Some(AcaciaDoor {
                r#open: false,
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14103 {
            return Some(AcaciaDoor {
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 14111 {
            return Some(AcaciaDoor {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14098 {
            return Some(AcaciaDoor {
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 14104 {
            return Some(AcaciaDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        return None;
    }
}
