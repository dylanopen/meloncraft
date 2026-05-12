use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatheredCopperDoor {
    pub r#hinge: Hinge,
    pub open: bool,
    pub powered: bool,
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

impl BlockState for WeatheredCopperDoor {
    fn to_id(self) -> i32 {
        if block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Upper { return 26065; }
        if block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#hinge == Hinge::Right { return 26020; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::North { return 26026; }
        if block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::West { return 26052; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == false && block_state.r#hinge == Hinge::Left { return 26024; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#open == false { return 26023; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#open == true { return 26014; }
        if block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::South { return 26037; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#half == Half::Lower { return 26027; }
        if block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#powered == false { return 26056; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right { return 26058; }
        if block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right && block_state.r#powered == true { return 26033; }
        if block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#facing == Facing::North && block_state.r#open == true { return 26018; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right { return 26041; }
        if block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Upper { return 26066; }
        if block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::West && block_state.r#open == true { return 26057; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Lower { return 26071; }
        if block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#open == true && block_state.r#half == Half::Upper { return 26045; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#facing == Facing::North { return 26017; }
        if block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::West { return 26047; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#powered == true && block_state.r#hinge == Hinge::Right { return 26035; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#powered == true { return 26051; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#open == false { return 26031; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Lower { return 26055; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#facing == Facing::West { return 26048; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Upper { return 26063; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper { return 26068; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::North { return 26028; }
        if block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Lower { return 26022; }
        if block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::West { return 26046; }
        if block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#powered == true { return 26067; }
        if block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left { return 26039; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::West { return 26054; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#half == Half::Upper { return 26030; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#open == true { return 26074; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#half == Half::Upper { return 26062; }
        if block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#powered == true { return 26013; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left { return 26015; }
        if block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::South && block_state.r#powered == true { return 26029; }
        if block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#powered == false { return 26040; }
        if block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#powered == false { return 26032; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#powered == true { return 26043; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#facing == Facing::North { return 26021; }
        if block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::West && block_state.r#open == false { return 26059; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper { return 26050; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#open == true { return 26061; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == false && block_state.r#hinge == Hinge::Left { return 26072; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == false { return 26070; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == false && block_state.r#hinge == Hinge::Right { return 26044; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#powered == false { return 26060; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#open == true { return 26034; }
        if block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower { return 26042; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#facing == Facing::East && block_state.r#open == false { return 26075; }
        if block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::South && block_state.r#powered == false { return 26038; }
        if block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#powered == false { return 26016; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == true { return 26025; }
        if block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#powered == true { return 26019; }
        if block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Upper { return 26049; }
        if block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right { return 26036; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#open == true { return 26069; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#half == Half::Lower { return 26073; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left { return 26053; }
        if block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#open == false && block_state.r#half == Half::Lower { return 26076; }
        if block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#open == false { return 26064; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26065 {
            return Some(WeatheredCopperDoor {
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Upper,
            });
        }
        if state_id == 26020 {
            return Some(WeatheredCopperDoor {
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26026 {
            return Some(WeatheredCopperDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 26052 {
            return Some(WeatheredCopperDoor {
                r#half: Half::Upper,
                r#powered: false,
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
            });
        }
        if state_id == 26024 {
            return Some(WeatheredCopperDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26023 {
            return Some(WeatheredCopperDoor {
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 26014 {
            return Some(WeatheredCopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 26037 {
            return Some(WeatheredCopperDoor {
                r#powered: true,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
            });
        }
        if state_id == 26027 {
            return Some(WeatheredCopperDoor {
                r#facing: Facing::North,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 26056 {
            return Some(WeatheredCopperDoor {
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 26058 {
            return Some(WeatheredCopperDoor {
                r#open: true,
                r#powered: false,
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26033 {
            return Some(WeatheredCopperDoor {
                r#half: Half::Upper,
                r#open: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#powered: true,
            });
        }
        if state_id == 26018 {
            return Some(WeatheredCopperDoor {
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 26041 {
            return Some(WeatheredCopperDoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26066 {
            return Some(WeatheredCopperDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 26057 {
            return Some(WeatheredCopperDoor {
                r#half: Half::Lower,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 26071 {
            return Some(WeatheredCopperDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Lower,
            });
        }
        if state_id == 26045 {
            return Some(WeatheredCopperDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#powered: true,
                r#open: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 26017 {
            return Some(WeatheredCopperDoor {
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#facing: Facing::North,
            });
        }
        if state_id == 26047 {
            return Some(WeatheredCopperDoor {
                r#half: Half::Upper,
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
            });
        }
        if state_id == 26035 {
            return Some(WeatheredCopperDoor {
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26051 {
            return Some(WeatheredCopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: false,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 26031 {
            return Some(WeatheredCopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 26055 {
            return Some(WeatheredCopperDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Lower,
            });
        }
        if state_id == 26048 {
            return Some(WeatheredCopperDoor {
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#facing: Facing::West,
            });
        }
        if state_id == 26063 {
            return Some(WeatheredCopperDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Upper,
            });
        }
        if state_id == 26068 {
            return Some(WeatheredCopperDoor {
                r#open: false,
                r#powered: false,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
            });
        }
        if state_id == 26028 {
            return Some(WeatheredCopperDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 26022 {
            return Some(WeatheredCopperDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#open: true,
                r#powered: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 26046 {
            return Some(WeatheredCopperDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: true,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 26067 {
            return Some(WeatheredCopperDoor {
                r#open: false,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 26039 {
            return Some(WeatheredCopperDoor {
                r#open: false,
                r#facing: Facing::South,
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26054 {
            return Some(WeatheredCopperDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 26030 {
            return Some(WeatheredCopperDoor {
                r#hinge: Hinge::Left,
                r#powered: false,
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 26074 {
            return Some(WeatheredCopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 26062 {
            return Some(WeatheredCopperDoor {
                r#powered: false,
                r#open: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#half: Half::Upper,
            });
        }
        if state_id == 26013 {
            return Some(WeatheredCopperDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26015 {
            return Some(WeatheredCopperDoor {
                r#open: false,
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26029 {
            return Some(WeatheredCopperDoor {
                r#half: Half::Upper,
                r#open: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 26040 {
            return Some(WeatheredCopperDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#open: false,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 26032 {
            return Some(WeatheredCopperDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26043 {
            return Some(WeatheredCopperDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#powered: true,
            });
        }
        if state_id == 26021 {
            return Some(WeatheredCopperDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26059 {
            return Some(WeatheredCopperDoor {
                r#half: Half::Lower,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 26050 {
            return Some(WeatheredCopperDoor {
                r#open: true,
                r#powered: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
            });
        }
        if state_id == 26061 {
            return Some(WeatheredCopperDoor {
                r#powered: true,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 26072 {
            return Some(WeatheredCopperDoor {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26070 {
            return Some(WeatheredCopperDoor {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 26044 {
            return Some(WeatheredCopperDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26060 {
            return Some(WeatheredCopperDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26034 {
            return Some(WeatheredCopperDoor {
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 26042 {
            return Some(WeatheredCopperDoor {
                r#hinge: Hinge::Right,
                r#powered: false,
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Lower,
            });
        }
        if state_id == 26075 {
            return Some(WeatheredCopperDoor {
                r#powered: true,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 26038 {
            return Some(WeatheredCopperDoor {
                r#open: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 26016 {
            return Some(WeatheredCopperDoor {
                r#open: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#powered: false,
            });
        }
        if state_id == 26025 {
            return Some(WeatheredCopperDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26019 {
            return Some(WeatheredCopperDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26049 {
            return Some(WeatheredCopperDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 26036 {
            return Some(WeatheredCopperDoor {
                r#half: Half::Upper,
                r#powered: false,
                r#open: false,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26069 {
            return Some(WeatheredCopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 26073 {
            return Some(WeatheredCopperDoor {
                r#facing: Facing::East,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 26053 {
            return Some(WeatheredCopperDoor {
                r#powered: true,
                r#facing: Facing::West,
                r#open: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26076 {
            return Some(WeatheredCopperDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#open: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 26064 {
            return Some(WeatheredCopperDoor {
                r#half: Half::Upper,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        return None;
    }
}

