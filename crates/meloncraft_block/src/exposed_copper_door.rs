use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposedCopperDoor {
    pub r#facing: Facing,
    pub r#half: Half,
    pub r#hinge: Hinge,
    pub open: bool,
    pub powered: bool,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hinge {
    Left,
    Right,
}

impl BlockState for ExposedCopperDoor {
    fn to_id(self) -> i32 {
        if block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#open == true && block_state.r#half == Half::Lower { return 25914; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#powered == true { return 25911; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#open == true { return 25938; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#powered == false { return 25908; }
        if block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left && block_state.r#open == true { return 25933; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#open == true { return 25921; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#open == true { return 25945; }
        if block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper { return 25902; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::South { return 25901; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#powered == false && block_state.r#hinge == Hinge::Right { return 25922; }
        if block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#half == Half::Upper { return 25924; }
        if block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#hinge == Hinge::Right { return 25931; }
        if block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#powered == false { return 25898; }
        if block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#powered == false { return 25894; }
        if block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left { return 25917; }
        if block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Upper { return 25937; }
        if block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South { return 25915; }
        if block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == true { return 25893; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#facing == Facing::West { return 25920; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::North && block_state.r#open == false { return 25887; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == false { return 25932; }
        if block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left { return 25925; }
        if block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Upper { return 25918; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == true { return 25895; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower { return 25926; }
        if block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#hinge == Hinge::Left { return 25909; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#half == Half::Lower { return 25944; }
        if block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left && block_state.r#powered == false { return 25936; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == true { return 25889; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#open == false { return 25907; }
        if block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Left && block_state.r#open == true { return 25910; }
        if block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left { return 25885; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper { return 25906; }
        if block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right && block_state.r#open == true { return 25905; }
        if block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#powered == false && block_state.r#half == Half::Lower { return 25948; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#facing == Facing::West { return 25919; }
        if block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#powered == true { return 25913; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#open == false { return 25896; }
        if block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#powered == true { return 25899; }
        if block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#hinge == Hinge::Right { return 25939; }
        if block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Lower { return 25929; }
        if block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right { return 25940; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::West { return 25923; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#powered == true { return 25941; }
        if block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::North { return 25888; }
        if block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#powered == false { return 25892; }
        if block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#powered == true { return 25903; }
        if block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#hinge == Hinge::Left { return 25943; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#open == false { return 25927; }
        if block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::North && block_state.r#open == false { return 25891; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#hinge == Hinge::Left { return 25904; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#powered == false { return 25900; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower { return 25912; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower && block_state.r#open == true { return 25942; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#open == true { return 25886; }
        if block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#half == Half::Lower { return 25947; }
        if block_state.r#powered == true && block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#facing == Facing::East { return 25935; }
        if block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right { return 25897; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right { return 25916; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == false { return 25946; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#open == true { return 25930; }
        if block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#open == true { return 25934; }
        if block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#powered == false { return 25928; }
        if block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#facing == Facing::North { return 25890; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25914 {
            return Some(ExposedCopperDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#powered: false,
                r#open: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 25911 {
            return Some(ExposedCopperDoor {
                r#hinge: Hinge::Left,
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#powered: true,
            });
        }
        if state_id == 25938 {
            return Some(ExposedCopperDoor {
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 25908 {
            return Some(ExposedCopperDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 25933 {
            return Some(ExposedCopperDoor {
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 25921 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 25945 {
            return Some(ExposedCopperDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 25902 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::South,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
            });
        }
        if state_id == 25901 {
            return Some(ExposedCopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 25922 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25924 {
            return Some(ExposedCopperDoor {
                r#powered: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 25931 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Lower,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25898 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::North,
                r#open: true,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 25894 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::North,
                r#open: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 25917 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::West,
                r#open: true,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25937 {
            return Some(ExposedCopperDoor {
                r#hinge: Hinge::Right,
                r#powered: true,
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Upper,
            });
        }
        if state_id == 25915 {
            return Some(ExposedCopperDoor {
                r#open: false,
                r#half: Half::Lower,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
            });
        }
        if state_id == 25893 {
            return Some(ExposedCopperDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 25920 {
            return Some(ExposedCopperDoor {
                r#powered: false,
                r#open: false,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#facing: Facing::West,
            });
        }
        if state_id == 25887 {
            return Some(ExposedCopperDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 25932 {
            return Some(ExposedCopperDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 25925 {
            return Some(ExposedCopperDoor {
                r#open: true,
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25918 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 25895 {
            return Some(ExposedCopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 25926 {
            return Some(ExposedCopperDoor {
                r#powered: false,
                r#open: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
            });
        }
        if state_id == 25909 {
            return Some(ExposedCopperDoor {
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25944 {
            return Some(ExposedCopperDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 25936 {
            return Some(ExposedCopperDoor {
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 25889 {
            return Some(ExposedCopperDoor {
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 25907 {
            return Some(ExposedCopperDoor {
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 25910 {
            return Some(ExposedCopperDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 25885 {
            return Some(ExposedCopperDoor {
                r#half: Half::Upper,
                r#powered: true,
                r#open: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25906 {
            return Some(ExposedCopperDoor {
                r#powered: false,
                r#open: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#half: Half::Upper,
            });
        }
        if state_id == 25905 {
            return Some(ExposedCopperDoor {
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 25948 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 25919 {
            return Some(ExposedCopperDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#open: false,
                r#half: Half::Upper,
                r#facing: Facing::West,
            });
        }
        if state_id == 25913 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 25896 {
            return Some(ExposedCopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 25899 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Lower,
                r#powered: true,
            });
        }
        if state_id == 25939 {
            return Some(ExposedCopperDoor {
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::East,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25929 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 25940 {
            return Some(ExposedCopperDoor {
                r#open: false,
                r#half: Half::Upper,
                r#powered: false,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25923 {
            return Some(ExposedCopperDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#open: false,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 25941 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 25888 {
            return Some(ExposedCopperDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: false,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 25892 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Upper,
                r#powered: false,
            });
        }
        if state_id == 25903 {
            return Some(ExposedCopperDoor {
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 25943 {
            return Some(ExposedCopperDoor {
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25927 {
            return Some(ExposedCopperDoor {
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 25891 {
            return Some(ExposedCopperDoor {
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 25904 {
            return Some(ExposedCopperDoor {
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25900 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: false,
            });
        }
        if state_id == 25912 {
            return Some(ExposedCopperDoor {
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Lower,
            });
        }
        if state_id == 25942 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::East,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: true,
            });
        }
        if state_id == 25886 {
            return Some(ExposedCopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 25947 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::East,
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 25935 {
            return Some(ExposedCopperDoor {
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#facing: Facing::East,
            });
        }
        if state_id == 25897 {
            return Some(ExposedCopperDoor {
                r#half: Half::Lower,
                r#powered: true,
                r#open: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25916 {
            return Some(ExposedCopperDoor {
                r#open: false,
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25946 {
            return Some(ExposedCopperDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 25930 {
            return Some(ExposedCopperDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#powered: false,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 25934 {
            return Some(ExposedCopperDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 25928 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 25890 {
            return Some(ExposedCopperDoor {
                r#half: Half::Upper,
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

