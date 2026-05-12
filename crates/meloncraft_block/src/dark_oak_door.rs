use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkOakDoor {
    pub open: bool,
    pub r#half: Half,
    pub r#facing: Facing,
    pub powered: bool,
    pub r#hinge: Hinge,
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

impl BlockState for DarkOakDoor {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#half == Half::Upper { return 14185; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#facing == Facing::North { return 14184; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#half == Half::Upper { return 14213; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#hinge == Hinge::Left { return 14218; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == true { return 14226; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left { return 14237; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#powered == false { return 14241; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#powered == true { return 14240; }
        if block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#facing == Facing::North { return 14182; }
        if block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#facing == Facing::South { return 14199; }
        if block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper { return 14214; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#open == true { return 14183; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == false { return 14212; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#powered == false && block_state.r#hinge == Hinge::Left { return 14179; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Upper { return 14228; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#powered == false { return 14225; }
        if block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#powered == true { return 14230; }
        if block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#powered == true { return 14196; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == true { return 14198; }
        if block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#facing == Facing::South { return 14207; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower { return 14222; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#open == true { return 14235; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#half == Half::Lower { return 14205; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#open == false { return 14209; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == false { return 14229; }
        if block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right { return 14232; }
        if block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::South && block_state.r#powered == false { return 14195; }
        if block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right { return 14217; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#open == false { return 14201; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#powered == true { return 14216; }
        if block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#hinge == Hinge::Right { return 14238; }
        if block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::West && block_state.r#open == true { return 14215; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#open == false { return 14236; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Left { return 14202; }
        if block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right && block_state.r#open == false { return 14208; }
        if block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#open == false { return 14200; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower { return 14223; }
        if block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#half == Half::Upper { return 14194; }
        if block_state.r#powered == true && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left { return 14180; }
        if block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#open == false { return 14189; }
        if block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East { return 14233; }
        if block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::South { return 14203; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == true { return 14224; }
        if block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North { return 14188; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right { return 14239; }
        if block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#facing == Facing::East { return 14231; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right { return 14190; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#facing == Facing::West { return 14210; }
        if block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::North { return 14193; }
        if block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#facing == Facing::North { return 14191; }
        if block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#hinge == Hinge::Left { return 14211; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::West && block_state.r#powered == true { return 14220; }
        if block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left { return 14234; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Lower { return 14219; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#facing == Facing::North { return 14186; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == false { return 14221; }
        if block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower { return 14206; }
        if block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper { return 14181; }
        if block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper { return 14178; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::North { return 14192; }
        if block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::South && block_state.r#open == false { return 14197; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == false { return 14227; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#open == true { return 14187; }
        if block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#hinge == Hinge::Left { return 14204; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14185 {
            return Some(DarkOakDoor {
                r#facing: Facing::North,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 14184 {
            return Some(DarkOakDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 14213 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Left,
                r#open: false,
                r#facing: Facing::West,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 14218 {
            return Some(DarkOakDoor {
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14226 {
            return Some(DarkOakDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 14237 {
            return Some(DarkOakDoor {
                r#facing: Facing::East,
                r#powered: false,
                r#open: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14241 {
            return Some(DarkOakDoor {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 14240 {
            return Some(DarkOakDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#powered: true,
            });
        }
        if state_id == 14182 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 14199 {
            return Some(DarkOakDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 14214 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Right,
                r#powered: true,
                r#open: true,
                r#facing: Facing::West,
                r#half: Half::Upper,
            });
        }
        if state_id == 14183 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 14212 {
            return Some(DarkOakDoor {
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 14179 {
            return Some(DarkOakDoor {
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14228 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#open: false,
                r#facing: Facing::East,
                r#half: Half::Upper,
            });
        }
        if state_id == 14225 {
            return Some(DarkOakDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 14230 {
            return Some(DarkOakDoor {
                r#open: true,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 14196 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 14198 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 14207 {
            return Some(DarkOakDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 14222 {
            return Some(DarkOakDoor {
                r#facing: Facing::West,
                r#powered: true,
                r#open: true,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
            });
        }
        if state_id == 14235 {
            return Some(DarkOakDoor {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 14205 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Left,
                r#powered: false,
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 14209 {
            return Some(DarkOakDoor {
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 14229 {
            return Some(DarkOakDoor {
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 14232 {
            return Some(DarkOakDoor {
                r#open: false,
                r#facing: Facing::East,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14195 {
            return Some(DarkOakDoor {
                r#open: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 14217 {
            return Some(DarkOakDoor {
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14201 {
            return Some(DarkOakDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 14216 {
            return Some(DarkOakDoor {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 14238 {
            return Some(DarkOakDoor {
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14215 {
            return Some(DarkOakDoor {
                r#half: Half::Upper,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 14236 {
            return Some(DarkOakDoor {
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 14202 {
            return Some(DarkOakDoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14208 {
            return Some(DarkOakDoor {
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 14200 {
            return Some(DarkOakDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Upper,
                r#open: false,
            });
        }
        if state_id == 14223 {
            return Some(DarkOakDoor {
                r#powered: false,
                r#facing: Facing::West,
                r#open: true,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
            });
        }
        if state_id == 14194 {
            return Some(DarkOakDoor {
                r#facing: Facing::South,
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 14180 {
            return Some(DarkOakDoor {
                r#powered: true,
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14189 {
            return Some(DarkOakDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 14233 {
            return Some(DarkOakDoor {
                r#half: Half::Upper,
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 14203 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#powered: false,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 14224 {
            return Some(DarkOakDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 14188 {
            return Some(DarkOakDoor {
                r#half: Half::Lower,
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 14239 {
            return Some(DarkOakDoor {
                r#facing: Facing::East,
                r#open: true,
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14231 {
            return Some(DarkOakDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 14190 {
            return Some(DarkOakDoor {
                r#facing: Facing::North,
                r#powered: true,
                r#open: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14210 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 14193 {
            return Some(DarkOakDoor {
                r#powered: false,
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::North,
            });
        }
        if state_id == 14191 {
            return Some(DarkOakDoor {
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 14211 {
            return Some(DarkOakDoor {
                r#half: Half::Upper,
                r#powered: false,
                r#facing: Facing::West,
                r#open: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14220 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Left,
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 14234 {
            return Some(DarkOakDoor {
                r#open: true,
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14219 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
                r#facing: Facing::West,
                r#half: Half::Lower,
            });
        }
        if state_id == 14186 {
            return Some(DarkOakDoor {
                r#powered: true,
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Lower,
                r#facing: Facing::North,
            });
        }
        if state_id == 14221 {
            return Some(DarkOakDoor {
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 14206 {
            return Some(DarkOakDoor {
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
            });
        }
        if state_id == 14181 {
            return Some(DarkOakDoor {
                r#powered: false,
                r#hinge: Hinge::Left,
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Upper,
            });
        }
        if state_id == 14178 {
            return Some(DarkOakDoor {
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Upper,
            });
        }
        if state_id == 14192 {
            return Some(DarkOakDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 14197 {
            return Some(DarkOakDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 14227 {
            return Some(DarkOakDoor {
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 14187 {
            return Some(DarkOakDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 14204 {
            return Some(DarkOakDoor {
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        return None;
    }
}

