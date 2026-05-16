use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkOakDoor {
    pub powered: bool,
    pub r#hinge: Hinge,
    pub open: bool,
    pub r#facing: Facing,
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

impl BlockState for DarkOakDoor {
    fn to_id(&self) -> i32 {
        if self.r#powered == true && self.r#open == false && self.r#half == Half::Lower && self.r#facing == Facing::South && self.r#hinge == Hinge::Right { return 14208; }
        if self.r#facing == Facing::North && self.r#hinge == Hinge::Left && self.r#half == Half::Lower && self.r#open == false && self.r#powered == true { return 14188; }
        if self.r#half == Half::Lower && self.r#open == false && self.r#facing == Facing::East && self.r#powered == true && self.r#hinge == Hinge::Left { return 14236; }
        if self.r#open == false && self.r#powered == false && self.r#hinge == Hinge::Right && self.r#half == Half::Lower && self.r#facing == Facing::North { return 14193; }
        if self.r#half == Half::Upper && self.r#open == false && self.r#facing == Facing::West && self.r#powered == true && self.r#hinge == Hinge::Right { return 14216; }
        if self.r#half == Half::Upper && self.r#powered == true && self.r#open == false && self.r#facing == Facing::North && self.r#hinge == Hinge::Right { return 14184; }
        if self.r#half == Half::Lower && self.r#powered == true && self.r#open == false && self.r#facing == Facing::West && self.r#hinge == Hinge::Right { return 14224; }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#open == true && self.r#hinge == Hinge::Left && self.r#half == Half::Upper { return 14178; }
        if self.r#half == Half::Lower && self.r#facing == Facing::South && self.r#hinge == Hinge::Left && self.r#open == false && self.r#powered == true { return 14204; }
        if self.r#half == Half::Upper && self.r#powered == true && self.r#hinge == Hinge::Left && self.r#facing == Facing::East && self.r#open == false { return 14228; }
        if self.r#hinge == Hinge::Right && self.r#open == false && self.r#powered == false && self.r#facing == Facing::West && self.r#half == Half::Upper { return 14217; }
        if self.r#hinge == Hinge::Right && self.r#facing == Facing::South && self.r#half == Half::Upper && self.r#open == false && self.r#powered == false { return 14201; }
        if self.r#facing == Facing::West && self.r#half == Half::Lower && self.r#open == true && self.r#powered == true && self.r#hinge == Hinge::Left { return 14218; }
        if self.r#powered == true && self.r#half == Half::Lower && self.r#hinge == Hinge::Right && self.r#open == true && self.r#facing == Facing::West { return 14222; }
        if self.r#hinge == Hinge::Right && self.r#open == false && self.r#half == Half::Lower && self.r#powered == true && self.r#facing == Facing::East { return 14240; }
        if self.r#powered == false && self.r#half == Half::Lower && self.r#open == true && self.r#facing == Facing::West && self.r#hinge == Hinge::Right { return 14223; }
        if self.r#facing == Facing::South && self.r#hinge == Hinge::Right && self.r#powered == false && self.r#open == true && self.r#half == Half::Lower { return 14207; }
        if self.r#facing == Facing::North && self.r#half == Half::Upper && self.r#open == false && self.r#hinge == Hinge::Right && self.r#powered == false { return 14185; }
        if self.r#facing == Facing::East && self.r#half == Half::Upper && self.r#powered == false && self.r#open == false && self.r#hinge == Hinge::Right { return 14233; }
        if self.r#open == true && self.r#half == Half::Lower && self.r#powered == true && self.r#facing == Facing::East && self.r#hinge == Hinge::Left { return 14234; }
        if self.r#half == Half::Upper && self.r#open == true && self.r#powered == true && self.r#facing == Facing::West && self.r#hinge == Hinge::Left { return 14210; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Right && self.r#facing == Facing::South && self.r#open == true && self.r#powered == true { return 14198; }
        if self.r#hinge == Hinge::Left && self.r#powered == true && self.r#open == true && self.r#facing == Facing::North && self.r#half == Half::Lower { return 14186; }
        if self.r#facing == Facing::North && self.r#hinge == Hinge::Right && self.r#open == true && self.r#half == Half::Lower && self.r#powered == false { return 14191; }
        if self.r#hinge == Hinge::Left && self.r#facing == Facing::West && self.r#half == Half::Upper && self.r#open == true && self.r#powered == false { return 14211; }
        if self.r#powered == false && self.r#hinge == Hinge::Right && self.r#facing == Facing::West && self.r#half == Half::Lower && self.r#open == false { return 14225; }
        if self.r#hinge == Hinge::Right && self.r#facing == Facing::East && self.r#half == Half::Lower && self.r#open == true && self.r#powered == false { return 14239; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#facing == Facing::West && self.r#open == false && self.r#powered == true { return 14212; }
        if self.r#facing == Facing::East && self.r#hinge == Hinge::Left && self.r#powered == false && self.r#open == true && self.r#half == Half::Lower { return 14235; }
        if self.r#open == true && self.r#facing == Facing::West && self.r#powered == false && self.r#hinge == Hinge::Left && self.r#half == Half::Lower { return 14219; }
        if self.r#powered == false && self.r#facing == Facing::North && self.r#open == true && self.r#half == Half::Upper && self.r#hinge == Hinge::Right { return 14183; }
        if self.r#facing == Facing::South && self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#open == true && self.r#powered == true { return 14194; }
        if self.r#open == true && self.r#powered == true && self.r#half == Half::Lower && self.r#facing == Facing::East && self.r#hinge == Hinge::Right { return 14238; }
        if self.r#facing == Facing::North && self.r#open == true && self.r#hinge == Hinge::Left && self.r#half == Half::Lower && self.r#powered == false { return 14187; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Right && self.r#open == true && self.r#facing == Facing::South && self.r#powered == false { return 14199; }
        if self.r#hinge == Hinge::Left && self.r#facing == Facing::South && self.r#open == false && self.r#powered == false && self.r#half == Half::Lower { return 14205; }
        if self.r#powered == true && self.r#half == Half::Upper && self.r#facing == Facing::West && self.r#hinge == Hinge::Right && self.r#open == true { return 14214; }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#hinge == Hinge::Right && self.r#open == true && self.r#half == Half::Lower { return 14206; }
        if self.r#facing == Facing::South && self.r#powered == false && self.r#hinge == Hinge::Left && self.r#open == true && self.r#half == Half::Lower { return 14203; }
        if self.r#hinge == Hinge::Right && self.r#facing == Facing::East && self.r#open == true && self.r#powered == false && self.r#half == Half::Upper { return 14231; }
        if self.r#open == true && self.r#powered == false && self.r#facing == Facing::East && self.r#hinge == Hinge::Left && self.r#half == Half::Upper { return 14227; }
        if self.r#powered == false && self.r#open == false && self.r#half == Half::Upper && self.r#facing == Facing::North && self.r#hinge == Hinge::Left { return 14181; }
        if self.r#half == Half::Upper && self.r#facing == Facing::South && self.r#open == false && self.r#hinge == Hinge::Left && self.r#powered == true { return 14196; }
        if self.r#powered == false && self.r#facing == Facing::West && self.r#hinge == Hinge::Left && self.r#half == Half::Upper && self.r#open == false { return 14213; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Right && self.r#facing == Facing::West && self.r#open == true && self.r#powered == false { return 14215; }
        if self.r#hinge == Hinge::Right && self.r#open == true && self.r#half == Half::Upper && self.r#facing == Facing::North && self.r#powered == true { return 14182; }
        if self.r#hinge == Hinge::Left && self.r#facing == Facing::South && self.r#powered == false && self.r#half == Half::Upper && self.r#open == false { return 14197; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Right && self.r#facing == Facing::East && self.r#open == false && self.r#powered == true { return 14232; }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#open == false { return 14180; }
        if self.r#half == Half::Lower && self.r#powered == true && self.r#open == true && self.r#facing == Facing::South && self.r#hinge == Hinge::Left { return 14202; }
        if self.r#hinge == Hinge::Left && self.r#facing == Facing::North && self.r#half == Half::Lower && self.r#open == false && self.r#powered == false { return 14189; }
        if self.r#hinge == Hinge::Right && self.r#open == true && self.r#half == Half::Upper && self.r#powered == true && self.r#facing == Facing::East { return 14230; }
        if self.r#powered == true && self.r#half == Half::Lower && self.r#open == true && self.r#facing == Facing::North && self.r#hinge == Hinge::Right { return 14190; }
        if self.r#facing == Facing::South && self.r#hinge == Hinge::Right && self.r#powered == true && self.r#open == false && self.r#half == Half::Upper { return 14200; }
        if self.r#hinge == Hinge::Left && self.r#open == false && self.r#powered == false && self.r#half == Half::Upper && self.r#facing == Facing::East { return 14229; }
        if self.r#half == Half::Upper && self.r#facing == Facing::North && self.r#powered == false && self.r#open == true && self.r#hinge == Hinge::Left { return 14179; }
        if self.r#hinge == Hinge::Left && self.r#powered == true && self.r#facing == Facing::West && self.r#half == Half::Lower && self.r#open == false { return 14220; }
        if self.r#facing == Facing::North && self.r#half == Half::Lower && self.r#powered == true && self.r#hinge == Hinge::Right && self.r#open == false { return 14192; }
        if self.r#facing == Facing::East && self.r#half == Half::Upper && self.r#open == true && self.r#powered == true && self.r#hinge == Hinge::Left { return 14226; }
        if self.r#half == Half::Lower && self.r#open == false && self.r#hinge == Hinge::Left && self.r#facing == Facing::East && self.r#powered == false { return 14237; }
        if self.r#facing == Facing::East && self.r#hinge == Hinge::Right && self.r#half == Half::Lower && self.r#open == false && self.r#powered == false { return 14241; }
        if self.r#hinge == Hinge::Right && self.r#half == Half::Lower && self.r#open == false && self.r#facing == Facing::South && self.r#powered == false { return 14209; }
        if self.r#facing == Facing::West && self.r#hinge == Hinge::Left && self.r#powered == false && self.r#half == Half::Lower && self.r#open == false { return 14221; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#facing == Facing::South && self.r#open == true && self.r#powered == false { return 14195; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14208 {
            return Some(DarkOakDoor {
                r#powered: true,
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14188 {
            return Some(DarkOakDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 14236 {
            return Some(DarkOakDoor {
                r#half: Half::Lower,
                r#open: false,
                r#facing: Facing::East,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14193 {
            return Some(DarkOakDoor {
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#facing: Facing::North,
            });
        }
        if state_id == 14216 {
            return Some(DarkOakDoor {
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::West,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14184 {
            return Some(DarkOakDoor {
                r#half: Half::Upper,
                r#powered: true,
                r#open: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14224 {
            return Some(DarkOakDoor {
                r#half: Half::Lower,
                r#powered: true,
                r#open: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14178 {
            return Some(DarkOakDoor {
                r#facing: Facing::North,
                r#powered: true,
                r#open: true,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
            });
        }
        if state_id == 14204 {
            return Some(DarkOakDoor {
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 14228 {
            return Some(DarkOakDoor {
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 14217 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
                r#facing: Facing::West,
                r#half: Half::Upper,
            });
        }
        if state_id == 14201 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 14218 {
            return Some(DarkOakDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14222 {
            return Some(DarkOakDoor {
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 14240 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 14223 {
            return Some(DarkOakDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14207 {
            return Some(DarkOakDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#open: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 14185 {
            return Some(DarkOakDoor {
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: false,
            });
        }
        if state_id == 14233 {
            return Some(DarkOakDoor {
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#powered: false,
                r#open: false,
                r#hinge: Hinge::Right,
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
        if state_id == 14210 {
            return Some(DarkOakDoor {
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14198 {
            return Some(DarkOakDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 14186 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Lower,
            });
        }
        if state_id == 14191 {
            return Some(DarkOakDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 14211 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 14225 {
            return Some(DarkOakDoor {
                r#powered: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 14239 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 14212 {
            return Some(DarkOakDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 14235 {
            return Some(DarkOakDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 14219 {
            return Some(DarkOakDoor {
                r#open: true,
                r#facing: Facing::West,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
            });
        }
        if state_id == 14183 {
            return Some(DarkOakDoor {
                r#powered: false,
                r#facing: Facing::North,
                r#open: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14194 {
            return Some(DarkOakDoor {
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 14238 {
            return Some(DarkOakDoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14187 {
            return Some(DarkOakDoor {
                r#facing: Facing::North,
                r#open: true,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 14199 {
            return Some(DarkOakDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 14205 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#open: false,
                r#powered: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 14214 {
            return Some(DarkOakDoor {
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 14206 {
            return Some(DarkOakDoor {
                r#facing: Facing::South,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 14203 {
            return Some(DarkOakDoor {
                r#facing: Facing::South,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 14231 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#open: true,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 14227 {
            return Some(DarkOakDoor {
                r#open: true,
                r#powered: false,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
            });
        }
        if state_id == 14181 {
            return Some(DarkOakDoor {
                r#powered: false,
                r#open: false,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14196 {
            return Some(DarkOakDoor {
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#open: false,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 14213 {
            return Some(DarkOakDoor {
                r#powered: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: false,
            });
        }
        if state_id == 14215 {
            return Some(DarkOakDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 14182 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 14197 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Upper,
                r#open: false,
            });
        }
        if state_id == 14232 {
            return Some(DarkOakDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 14180 {
            return Some(DarkOakDoor {
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 14202 {
            return Some(DarkOakDoor {
                r#half: Half::Lower,
                r#powered: true,
                r#open: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14189 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 14230 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 14190 {
            return Some(DarkOakDoor {
                r#powered: true,
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 14200 {
            return Some(DarkOakDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#open: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 14229 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::East,
            });
        }
        if state_id == 14179 {
            return Some(DarkOakDoor {
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#powered: false,
                r#open: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14220 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 14192 {
            return Some(DarkOakDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 14226 {
            return Some(DarkOakDoor {
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 14237 {
            return Some(DarkOakDoor {
                r#half: Half::Lower,
                r#open: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 14241 {
            return Some(DarkOakDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 14209 {
            return Some(DarkOakDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: false,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 14221 {
            return Some(DarkOakDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 14195 {
            return Some(DarkOakDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#open: true,
                r#powered: false,
            });
        }
        return None;
    }
}

