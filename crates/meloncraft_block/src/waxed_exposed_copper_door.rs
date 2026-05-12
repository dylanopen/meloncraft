use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedExposedCopperDoor {
    pub r#hinge: Hinge,
    pub open: bool,
    pub r#facing: Facing,
    pub r#half: Half,
    pub powered: bool,
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

impl BlockState for WaxedExposedCopperDoor {
    fn to_id(self) -> i32 {
        if block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left { return 26142; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right { return 26188; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#powered == false { return 26176; }
        if block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#facing == Facing::North { return 26147; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower && block_state.r#open == true { return 26150; }
        if block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#facing == Facing::South && block_state.r#powered == true { return 26169; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#powered == true { return 26165; }
        if block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper { return 26158; }
        if block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#half == Half::Upper { return 26191; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper { return 26161; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#powered == false { return 26204; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#open == false { return 26168; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::West { return 26186; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#open == true { return 26185; }
        if block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#open == true && block_state.r#half == Half::Upper { return 26146; }
        if block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#open == false { return 26200; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#hinge == Hinge::Left { return 26190; }
        if block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#powered == true { return 26197; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#powered == false { return 26192; }
        if block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#open == false { return 26199; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper && block_state.r#open == false { return 26144; }
        if block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#open == true { return 26198; }
        if block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#hinge == Hinge::Right { return 26201; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == true { return 26145; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::West && block_state.r#open == true { return 26182; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#powered == true { return 26193; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#open == true { return 26173; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#powered == false && block_state.r#hinge == Hinge::Right { return 26154; }
        if block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left && block_state.r#open == false { return 26151; }
        if block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#powered == true { return 26159; }
        if block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East { return 26195; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == true && block_state.r#hinge == Hinge::Right { return 26203; }
        if block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left && block_state.r#powered == false { return 26184; }
        if block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::West { return 26175; }
        if block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#facing == Facing::North { return 26148; }
        if block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right && block_state.r#open == true { return 26162; }
        if block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::South { return 26160; }
        if block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South { return 26172; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#powered == true { return 26177; }
        if block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right && block_state.r#powered == false { return 26194; }
        if block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#hinge == Hinge::Right { return 26156; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Upper { return 26196; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right { return 26179; }
        if block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == true { return 26167; }
        if block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Lower { return 26153; }
        if block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#facing == Facing::West { return 26180; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#open == false && block_state.r#hinge == Hinge::Right { return 26164; }
        if block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#powered == true { return 26157; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#open == true && block_state.r#hinge == Hinge::Left { return 26149; }
        if block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left { return 26174; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#facing == Facing::South { return 26163; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper && block_state.r#open == true { return 26141; }
        if block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right { return 26171; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#open == true { return 26178; }
        if block_state.r#powered == true && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::West && block_state.r#half == Half::Lower { return 26181; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right { return 26170; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#open == true { return 26189; }
        if block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#hinge == Hinge::Left { return 26152; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#powered == false && block_state.r#hinge == Hinge::Left { return 26166; }
        if block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right { return 26202; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#open == false && block_state.r#hinge == Hinge::Left { return 26183; }
        if block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left { return 26143; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#powered == true { return 26155; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#powered == true { return 26187; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26142 {
            return Some(WaxedExposedCopperDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#open: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26188 {
            return Some(WaxedExposedCopperDoor {
                r#powered: false,
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26176 {
            return Some(WaxedExposedCopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 26147 {
            return Some(WaxedExposedCopperDoor {
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 26150 {
            return Some(WaxedExposedCopperDoor {
                r#powered: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: true,
            });
        }
        if state_id == 26169 {
            return Some(WaxedExposedCopperDoor {
                r#open: true,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 26165 {
            return Some(WaxedExposedCopperDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 26158 {
            return Some(WaxedExposedCopperDoor {
                r#facing: Facing::South,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
            });
        }
        if state_id == 26191 {
            return Some(WaxedExposedCopperDoor {
                r#facing: Facing::East,
                r#open: false,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 26161 {
            return Some(WaxedExposedCopperDoor {
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#half: Half::Upper,
            });
        }
        if state_id == 26204 {
            return Some(WaxedExposedCopperDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26168 {
            return Some(WaxedExposedCopperDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 26186 {
            return Some(WaxedExposedCopperDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: true,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 26185 {
            return Some(WaxedExposedCopperDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 26146 {
            return Some(WaxedExposedCopperDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#powered: false,
                r#open: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 26200 {
            return Some(WaxedExposedCopperDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 26190 {
            return Some(WaxedExposedCopperDoor {
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Upper,
                r#powered: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26197 {
            return Some(WaxedExposedCopperDoor {
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 26192 {
            return Some(WaxedExposedCopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26199 {
            return Some(WaxedExposedCopperDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 26144 {
            return Some(WaxedExposedCopperDoor {
                r#hinge: Hinge::Left,
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#open: false,
            });
        }
        if state_id == 26198 {
            return Some(WaxedExposedCopperDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#half: Half::Lower,
                r#open: true,
            });
        }
        if state_id == 26201 {
            return Some(WaxedExposedCopperDoor {
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::East,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26145 {
            return Some(WaxedExposedCopperDoor {
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26182 {
            return Some(WaxedExposedCopperDoor {
                r#hinge: Hinge::Left,
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 26193 {
            return Some(WaxedExposedCopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26173 {
            return Some(WaxedExposedCopperDoor {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 26154 {
            return Some(WaxedExposedCopperDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26151 {
            return Some(WaxedExposedCopperDoor {
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 26159 {
            return Some(WaxedExposedCopperDoor {
                r#open: false,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 26195 {
            return Some(WaxedExposedCopperDoor {
                r#open: false,
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 26203 {
            return Some(WaxedExposedCopperDoor {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26184 {
            return Some(WaxedExposedCopperDoor {
                r#half: Half::Lower,
                r#open: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 26175 {
            return Some(WaxedExposedCopperDoor {
                r#open: false,
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
            });
        }
        if state_id == 26148 {
            return Some(WaxedExposedCopperDoor {
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Upper,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 26162 {
            return Some(WaxedExposedCopperDoor {
                r#half: Half::Upper,
                r#powered: false,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 26160 {
            return Some(WaxedExposedCopperDoor {
                r#open: false,
                r#half: Half::Upper,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
            });
        }
        if state_id == 26172 {
            return Some(WaxedExposedCopperDoor {
                r#open: false,
                r#half: Half::Lower,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
            });
        }
        if state_id == 26177 {
            return Some(WaxedExposedCopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26194 {
            return Some(WaxedExposedCopperDoor {
                r#half: Half::Upper,
                r#open: true,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#powered: false,
            });
        }
        if state_id == 26156 {
            return Some(WaxedExposedCopperDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#open: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26196 {
            return Some(WaxedExposedCopperDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Upper,
            });
        }
        if state_id == 26179 {
            return Some(WaxedExposedCopperDoor {
                r#open: false,
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26167 {
            return Some(WaxedExposedCopperDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26153 {
            return Some(WaxedExposedCopperDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 26180 {
            return Some(WaxedExposedCopperDoor {
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 26164 {
            return Some(WaxedExposedCopperDoor {
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#powered: false,
                r#open: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26157 {
            return Some(WaxedExposedCopperDoor {
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 26149 {
            return Some(WaxedExposedCopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#powered: true,
                r#open: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26174 {
            return Some(WaxedExposedCopperDoor {
                r#open: true,
                r#half: Half::Upper,
                r#powered: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26163 {
            return Some(WaxedExposedCopperDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 26141 {
            return Some(WaxedExposedCopperDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 26171 {
            return Some(WaxedExposedCopperDoor {
                r#half: Half::Lower,
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26178 {
            return Some(WaxedExposedCopperDoor {
                r#facing: Facing::West,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 26181 {
            return Some(WaxedExposedCopperDoor {
                r#powered: true,
                r#open: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#half: Half::Lower,
            });
        }
        if state_id == 26170 {
            return Some(WaxedExposedCopperDoor {
                r#powered: false,
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26189 {
            return Some(WaxedExposedCopperDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 26152 {
            return Some(WaxedExposedCopperDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26166 {
            return Some(WaxedExposedCopperDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26202 {
            return Some(WaxedExposedCopperDoor {
                r#half: Half::Lower,
                r#open: true,
                r#powered: false,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26183 {
            return Some(WaxedExposedCopperDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26143 {
            return Some(WaxedExposedCopperDoor {
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26155 {
            return Some(WaxedExposedCopperDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26187 {
            return Some(WaxedExposedCopperDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: true,
            });
        }
        return None;
    }
}

