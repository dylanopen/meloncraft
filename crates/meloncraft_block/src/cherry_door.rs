use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherryDoor {
    pub r#half: Half,
    pub r#hinge: Hinge,
    pub r#facing: Facing,
    pub powered: bool,
    pub open: bool,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for CherryDoor {
    fn to_id(self) -> i32 {
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left { return 14116; }
        if block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left { return 14170; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#powered == true { return 14176; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left { return 14140; }
        if block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#hinge == Hinge::Right { return 14120; }
        if block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#powered == true { return 14130; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#open == true { return 14171; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#open == true && block_state.r#hinge == Hinge::Left { return 14163; }
        if block_state.r#powered == true && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::West { return 14152; }
        if block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Lower { return 14154; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#powered == true { return 14126; }
        if block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#facing == Facing::West { return 14159; }
        if block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#open == false { return 14160; }
        if block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Lower { return 14138; }
        if block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#facing == Facing::South { return 14143; }
        if block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#powered == true { return 14146; }
        if block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#open == true { return 14151; }
        if block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#facing == Facing::North { return 14125; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper { return 14132; }
        if block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#open == true && block_state.r#half == Half::Lower { return 14158; }
        if block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#hinge == Hinge::Left { return 14165; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#powered == false { return 14175; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#half == Half::Upper { return 14119; }
        if block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#open == true { return 14115; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#open == false && block_state.r#facing == Facing::East { return 14164; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == true { return 14162; }
        if block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower { return 14173; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#open == false { return 14133; }
        if block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::West { return 14156; }
        if block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#half == Half::Upper { return 14166; }
        if block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East { return 14172; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#powered == true && block_state.r#hinge == Hinge::Right { return 14118; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == true { return 14142; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Lower { return 14122; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#open == true && block_state.r#hinge == Hinge::Right { return 14135; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#open == false { return 14128; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right { return 14174; }
        if block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == false && block_state.r#half == Half::Lower { return 14161; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#powered == false { return 14157; }
        if block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left { return 14117; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#open == true { return 14134; }
        if block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#half == Half::Lower { return 14145; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::West { return 14148; }
        if block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::South && block_state.r#powered == false { return 14131; }
        if block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#open == false { return 14153; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#facing == Facing::North { return 14124; }
        if block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right { return 14150; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#powered == false && block_state.r#hinge == Hinge::Right { return 14167; }
        if block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::North { return 14129; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#hinge == Hinge::Left { return 14141; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#hinge == Hinge::Right { return 14136; }
        if block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#powered == false { return 14147; }
        if block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left { return 14149; }
        if block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#facing == Facing::South { return 14139; }
        if block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right && block_state.r#open == false { return 14144; }
        if block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::West { return 14155; }
        if block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Upper { return 14168; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper { return 14121; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#powered == false { return 14137; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#open == true { return 14123; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Upper { return 14169; }
        if block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left && block_state.r#open == true { return 14114; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#open == false { return 14177; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#open == true { return 14127; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14116 {
            return Some(CherryDoor {
                r#powered: true,
                r#facing: Facing::North,
                r#open: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14170 {
            return Some(CherryDoor {
                r#open: true,
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14176 {
            return Some(CherryDoor {
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: true,
            });
        }
        if state_id == 14140 {
            return Some(CherryDoor {
                r#powered: true,
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14120 {
            return Some(CherryDoor {
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14130 {
            return Some(CherryDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: true,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 14171 {
            return Some(CherryDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 14163 {
            return Some(CherryDoor {
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#powered: false,
                r#open: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14152 {
            return Some(CherryDoor {
                r#powered: true,
                r#open: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
            });
        }
        if state_id == 14154 {
            return Some(CherryDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 14126 {
            return Some(CherryDoor {
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: true,
            });
        }
        if state_id == 14159 {
            return Some(CherryDoor {
                r#hinge: Hinge::Right,
                r#powered: false,
                r#open: true,
                r#half: Half::Lower,
                r#facing: Facing::West,
            });
        }
        if state_id == 14160 {
            return Some(CherryDoor {
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 14138 {
            return Some(CherryDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 14143 {
            return Some(CherryDoor {
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 14146 {
            return Some(CherryDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 14151 {
            return Some(CherryDoor {
                r#hinge: Hinge::Right,
                r#powered: false,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 14125 {
            return Some(CherryDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 14132 {
            return Some(CherryDoor {
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Upper,
            });
        }
        if state_id == 14158 {
            return Some(CherryDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#open: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 14165 {
            return Some(CherryDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14175 {
            return Some(CherryDoor {
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#powered: false,
            });
        }
        if state_id == 14119 {
            return Some(CherryDoor {
                r#facing: Facing::North,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 14115 {
            return Some(CherryDoor {
                r#half: Half::Upper,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 14164 {
            return Some(CherryDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#open: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 14162 {
            return Some(CherryDoor {
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 14173 {
            return Some(CherryDoor {
                r#facing: Facing::East,
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
            });
        }
        if state_id == 14133 {
            return Some(CherryDoor {
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 14156 {
            return Some(CherryDoor {
                r#half: Half::Lower,
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
            });
        }
        if state_id == 14166 {
            return Some(CherryDoor {
                r#hinge: Hinge::Right,
                r#open: true,
                r#facing: Facing::East,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 14172 {
            return Some(CherryDoor {
                r#powered: true,
                r#half: Half::Lower,
                r#open: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 14118 {
            return Some(CherryDoor {
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14142 {
            return Some(CherryDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 14122 {
            return Some(CherryDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Lower,
            });
        }
        if state_id == 14135 {
            return Some(CherryDoor {
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#powered: false,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14128 {
            return Some(CherryDoor {
                r#facing: Facing::North,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 14174 {
            return Some(CherryDoor {
                r#facing: Facing::East,
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14161 {
            return Some(CherryDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 14157 {
            return Some(CherryDoor {
                r#hinge: Hinge::Left,
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 14117 {
            return Some(CherryDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14134 {
            return Some(CherryDoor {
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 14145 {
            return Some(CherryDoor {
                r#hinge: Hinge::Right,
                r#open: false,
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 14148 {
            return Some(CherryDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::West,
            });
        }
        if state_id == 14131 {
            return Some(CherryDoor {
                r#open: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 14153 {
            return Some(CherryDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 14124 {
            return Some(CherryDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Lower,
                r#open: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 14150 {
            return Some(CherryDoor {
                r#powered: true,
                r#open: true,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14167 {
            return Some(CherryDoor {
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14129 {
            return Some(CherryDoor {
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::North,
            });
        }
        if state_id == 14141 {
            return Some(CherryDoor {
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Lower,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14136 {
            return Some(CherryDoor {
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#open: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14147 {
            return Some(CherryDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 14149 {
            return Some(CherryDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14139 {
            return Some(CherryDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 14144 {
            return Some(CherryDoor {
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 14155 {
            return Some(CherryDoor {
                r#half: Half::Lower,
                r#powered: false,
                r#open: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
            });
        }
        if state_id == 14168 {
            return Some(CherryDoor {
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Upper,
            });
        }
        if state_id == 14121 {
            return Some(CherryDoor {
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#half: Half::Upper,
            });
        }
        if state_id == 14137 {
            return Some(CherryDoor {
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 14123 {
            return Some(CherryDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 14169 {
            return Some(CherryDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Upper,
            });
        }
        if state_id == 14114 {
            return Some(CherryDoor {
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 14177 {
            return Some(CherryDoor {
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 14127 {
            return Some(CherryDoor {
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        return None;
    }
}

