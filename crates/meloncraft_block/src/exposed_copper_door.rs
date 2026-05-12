use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposedCopperDoor {
    pub open: bool,
    pub powered: bool,
    pub r#facing: Facing,
    pub r#half: Half,
    pub r#hinge: Hinge,
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
    fn to_id(&self) -> i32 {
        if self.r#powered == true && self.r#half == Half::Upper && self.r#facing == Facing::East && self.r#hinge == Hinge::Right && self.r#open == true { return 25937; }
        if self.r#open == true && self.r#powered == false && self.r#hinge == Hinge::Right && self.r#facing == Facing::East && self.r#half == Half::Upper { return 25938; }
        if self.r#facing == Facing::East && self.r#hinge == Hinge::Right && self.r#half == Half::Lower && self.r#open == false && self.r#powered == false { return 25948; }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#hinge == Hinge::Right && self.r#half == Half::Upper && self.r#open == true { return 25905; }
        if self.r#half == Half::Upper && self.r#open == true && self.r#powered == true && self.r#hinge == Hinge::Left && self.r#facing == Facing::East { return 25933; }
        if self.r#half == Half::Lower && self.r#powered == false && self.r#open == true && self.r#hinge == Hinge::Right && self.r#facing == Facing::North { return 25898; }
        if self.r#open == true && self.r#hinge == Hinge::Left && self.r#powered == false && self.r#facing == Facing::North && self.r#half == Half::Lower { return 25894; }
        if self.r#half == Half::Upper && self.r#open == true && self.r#powered == false && self.r#hinge == Hinge::Left && self.r#facing == Facing::North { return 25886; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#hinge == Hinge::Right && self.r#open == false && self.r#half == Half::Upper { return 25908; }
        if self.r#hinge == Hinge::Left && self.r#open == false && self.r#powered == true && self.r#facing == Facing::South && self.r#half == Half::Upper { return 25903; }
        if self.r#hinge == Hinge::Right && self.r#open == false && self.r#powered == true && self.r#half == Half::Lower && self.r#facing == Facing::North { return 25899; }
        if self.r#powered == false && self.r#half == Half::Lower && self.r#hinge == Hinge::Right && self.r#open == true && self.r#facing == Facing::South { return 25914; }
        if self.r#facing == Facing::West && self.r#half == Half::Upper && self.r#powered == true && self.r#open == true && self.r#hinge == Hinge::Right { return 25921; }
        if self.r#hinge == Hinge::Right && self.r#open == true && self.r#powered == false && self.r#facing == Facing::East && self.r#half == Half::Lower { return 25946; }
        if self.r#powered == true && self.r#facing == Facing::East && self.r#hinge == Hinge::Left && self.r#half == Half::Upper && self.r#open == false { return 25935; }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#open == false && self.r#hinge == Hinge::Left && self.r#half == Half::Upper { return 25887; }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#open == true && self.r#half == Half::Lower && self.r#hinge == Hinge::Right { return 25913; }
        if self.r#half == Half::Lower && self.r#facing == Facing::West && self.r#hinge == Hinge::Right && self.r#open == true && self.r#powered == true { return 25929; }
        if self.r#open == false && self.r#facing == Facing::South && self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#powered == true { return 25911; }
        if self.r#facing == Facing::East && self.r#half == Half::Upper && self.r#powered == false && self.r#open == false && self.r#hinge == Hinge::Left { return 25936; }
        if self.r#facing == Facing::North && self.r#hinge == Hinge::Right && self.r#open == true && self.r#powered == true && self.r#half == Half::Upper { return 25889; }
        if self.r#facing == Facing::South && self.r#hinge == Hinge::Left && self.r#powered == true && self.r#open == true && self.r#half == Half::Upper { return 25901; }
        if self.r#facing == Facing::West && self.r#hinge == Hinge::Left && self.r#powered == false && self.r#half == Half::Upper && self.r#open == true { return 25918; }
        if self.r#facing == Facing::West && self.r#half == Half::Upper && self.r#powered == false && self.r#hinge == Hinge::Left && self.r#open == false { return 25920; }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#hinge == Hinge::Right && self.r#half == Half::Lower && self.r#open == true { return 25897; }
        if self.r#facing == Facing::West && self.r#powered == false && self.r#open == true && self.r#half == Half::Lower && self.r#hinge == Hinge::Left { return 25926; }
        if self.r#powered == true && self.r#facing == Facing::North && self.r#open == false && self.r#half == Half::Upper && self.r#hinge == Hinge::Right { return 25891; }
        if self.r#half == Half::Upper && self.r#powered == true && self.r#facing == Facing::West && self.r#open == false && self.r#hinge == Hinge::Left { return 25919; }
        if self.r#facing == Facing::East && self.r#open == false && self.r#half == Half::Upper && self.r#hinge == Hinge::Right && self.r#powered == true { return 25939; }
        if self.r#facing == Facing::North && self.r#hinge == Hinge::Left && self.r#powered == false && self.r#open == false && self.r#half == Half::Upper { return 25888; }
        if self.r#facing == Facing::East && self.r#powered == true && self.r#half == Half::Lower && self.r#open == false && self.r#hinge == Hinge::Left { return 25943; }
        if self.r#half == Half::Upper && self.r#open == false && self.r#powered == true && self.r#facing == Facing::West && self.r#hinge == Hinge::Right { return 25923; }
        if self.r#hinge == Hinge::Right && self.r#facing == Facing::West && self.r#half == Half::Lower && self.r#open == false && self.r#powered == false { return 25932; }
        if self.r#facing == Facing::East && self.r#open == true && self.r#powered == true && self.r#hinge == Hinge::Left && self.r#half == Half::Lower { return 25941; }
        if self.r#half == Half::Upper && self.r#open == true && self.r#hinge == Hinge::Right && self.r#powered == false && self.r#facing == Facing::South { return 25906; }
        if self.r#hinge == Hinge::Left && self.r#half == Half::Upper && self.r#open == false && self.r#facing == Facing::South && self.r#powered == false { return 25904; }
        if self.r#half == Half::Lower && self.r#hinge == Hinge::Left && self.r#powered == false && self.r#open == true && self.r#facing == Facing::East { return 25942; }
        if self.r#facing == Facing::East && self.r#powered == true && self.r#open == true && self.r#hinge == Hinge::Right && self.r#half == Half::Lower { return 25945; }
        if self.r#hinge == Hinge::Left && self.r#open == true && self.r#powered == true && self.r#facing == Facing::West && self.r#half == Half::Upper { return 25917; }
        if self.r#powered == false && self.r#hinge == Hinge::Left && self.r#half == Half::Lower && self.r#open == false && self.r#facing == Facing::South { return 25912; }
        if self.r#powered == true && self.r#facing == Facing::North && self.r#half == Half::Lower && self.r#open == true && self.r#hinge == Hinge::Left { return 25893; }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#open == false && self.r#hinge == Hinge::Right && self.r#half == Half::Lower { return 25915; }
        if self.r#half == Half::Lower && self.r#powered == true && self.r#open == true && self.r#hinge == Hinge::Left && self.r#facing == Facing::West { return 25925; }
        if self.r#facing == Facing::East && self.r#hinge == Hinge::Left && self.r#half == Half::Lower && self.r#open == false && self.r#powered == false { return 25944; }
        if self.r#open == false && self.r#half == Half::Upper && self.r#hinge == Hinge::Right && self.r#facing == Facing::North && self.r#powered == false { return 25892; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#open == true && self.r#half == Half::Upper && self.r#hinge == Hinge::Left { return 25902; }
        if self.r#half == Half::Lower && self.r#hinge == Hinge::Right && self.r#open == false && self.r#powered == false && self.r#facing == Facing::South { return 25916; }
        if self.r#hinge == Hinge::Left && self.r#facing == Facing::North && self.r#open == false && self.r#powered == false && self.r#half == Half::Lower { return 25896; }
        if self.r#facing == Facing::North && self.r#half == Half::Lower && self.r#powered == true && self.r#open == false && self.r#hinge == Hinge::Left { return 25895; }
        if self.r#powered == false && self.r#half == Half::Upper && self.r#open == true && self.r#hinge == Hinge::Right && self.r#facing == Facing::North { return 25890; }
        if self.r#facing == Facing::South && self.r#hinge == Hinge::Left && self.r#powered == false && self.r#open == true && self.r#half == Half::Lower { return 25910; }
        if self.r#open == false && self.r#half == Half::Lower && self.r#facing == Facing::West && self.r#powered == false && self.r#hinge == Hinge::Left { return 25928; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#open == true && self.r#powered == true && self.r#facing == Facing::North { return 25885; }
        if self.r#facing == Facing::West && self.r#hinge == Hinge::Right && self.r#open == true && self.r#half == Half::Lower && self.r#powered == false { return 25930; }
        if self.r#half == Half::Upper && self.r#powered == false && self.r#facing == Facing::West && self.r#hinge == Hinge::Right && self.r#open == false { return 25924; }
        if self.r#powered == true && self.r#open == false && self.r#half == Half::Lower && self.r#hinge == Hinge::Right && self.r#facing == Facing::East { return 25947; }
        if self.r#half == Half::Lower && self.r#open == false && self.r#facing == Facing::North && self.r#hinge == Hinge::Right && self.r#powered == false { return 25900; }
        if self.r#facing == Facing::South && self.r#powered == true && self.r#hinge == Hinge::Left && self.r#half == Half::Lower && self.r#open == true { return 25909; }
        if self.r#open == true && self.r#powered == false && self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#facing == Facing::East { return 25934; }
        if self.r#open == true && self.r#powered == false && self.r#facing == Facing::West && self.r#half == Half::Upper && self.r#hinge == Hinge::Right { return 25922; }
        if self.r#facing == Facing::West && self.r#hinge == Hinge::Right && self.r#open == false && self.r#powered == true && self.r#half == Half::Lower { return 25931; }
        if self.r#facing == Facing::South && self.r#half == Half::Upper && self.r#hinge == Hinge::Right && self.r#open == false && self.r#powered == true { return 25907; }
        if self.r#facing == Facing::West && self.r#hinge == Hinge::Left && self.r#open == false && self.r#half == Half::Lower && self.r#powered == true { return 25927; }
        if self.r#powered == false && self.r#half == Half::Upper && self.r#open == false && self.r#facing == Facing::East && self.r#hinge == Hinge::Right { return 25940; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25937 {
            return Some(ExposedCopperDoor {
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 25938 {
            return Some(ExposedCopperDoor {
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#half: Half::Upper,
            });
        }
        if state_id == 25948 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 25905 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::South,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 25933 {
            return Some(ExposedCopperDoor {
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 25898 {
            return Some(ExposedCopperDoor {
                r#half: Half::Lower,
                r#powered: false,
                r#open: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 25894 {
            return Some(ExposedCopperDoor {
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Lower,
            });
        }
        if state_id == 25886 {
            return Some(ExposedCopperDoor {
                r#half: Half::Upper,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 25908 {
            return Some(ExposedCopperDoor {
                r#powered: false,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 25903 {
            return Some(ExposedCopperDoor {
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Upper,
            });
        }
        if state_id == 25899 {
            return Some(ExposedCopperDoor {
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::North,
            });
        }
        if state_id == 25914 {
            return Some(ExposedCopperDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 25921 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#powered: true,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25946 {
            return Some(ExposedCopperDoor {
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Lower,
            });
        }
        if state_id == 25935 {
            return Some(ExposedCopperDoor {
                r#powered: true,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: false,
            });
        }
        if state_id == 25887 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::North,
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
            });
        }
        if state_id == 25913 {
            return Some(ExposedCopperDoor {
                r#powered: true,
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25929 {
            return Some(ExposedCopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 25911 {
            return Some(ExposedCopperDoor {
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 25936 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#powered: false,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25889 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 25901 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#open: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 25918 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 25920 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 25897 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::North,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: true,
            });
        }
        if state_id == 25926 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::West,
                r#powered: false,
                r#open: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25891 {
            return Some(ExposedCopperDoor {
                r#powered: true,
                r#facing: Facing::North,
                r#open: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25919 {
            return Some(ExposedCopperDoor {
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::West,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25939 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#powered: true,
            });
        }
        if state_id == 25888 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 25943 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::East,
                r#powered: true,
                r#half: Half::Lower,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25923 {
            return Some(ExposedCopperDoor {
                r#half: Half::Upper,
                r#open: false,
                r#powered: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25932 {
            return Some(ExposedCopperDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 25941 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::East,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
            });
        }
        if state_id == 25906 {
            return Some(ExposedCopperDoor {
                r#half: Half::Upper,
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 25904 {
            return Some(ExposedCopperDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 25942 {
            return Some(ExposedCopperDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 25945 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::East,
                r#powered: true,
                r#open: true,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
            });
        }
        if state_id == 25917 {
            return Some(ExposedCopperDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Upper,
            });
        }
        if state_id == 25912 {
            return Some(ExposedCopperDoor {
                r#powered: false,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 25893 {
            return Some(ExposedCopperDoor {
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25915 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::South,
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
            });
        }
        if state_id == 25925 {
            return Some(ExposedCopperDoor {
                r#half: Half::Lower,
                r#powered: true,
                r#open: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
            });
        }
        if state_id == 25944 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 25892 {
            return Some(ExposedCopperDoor {
                r#open: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 25902 {
            return Some(ExposedCopperDoor {
                r#powered: false,
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25916 {
            return Some(ExposedCopperDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 25896 {
            return Some(ExposedCopperDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#open: false,
                r#powered: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 25895 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25890 {
            return Some(ExposedCopperDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#open: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 25910 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 25928 {
            return Some(ExposedCopperDoor {
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#powered: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25885 {
            return Some(ExposedCopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 25930 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 25924 {
            return Some(ExposedCopperDoor {
                r#half: Half::Upper,
                r#powered: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 25947 {
            return Some(ExposedCopperDoor {
                r#powered: true,
                r#open: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 25900 {
            return Some(ExposedCopperDoor {
                r#half: Half::Lower,
                r#open: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#powered: false,
            });
        }
        if state_id == 25909 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::South,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: true,
            });
        }
        if state_id == 25934 {
            return Some(ExposedCopperDoor {
                r#open: true,
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 25922 {
            return Some(ExposedCopperDoor {
                r#open: true,
                r#powered: false,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25931 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 25907 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 25927 {
            return Some(ExposedCopperDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#open: false,
                r#half: Half::Lower,
                r#powered: true,
            });
        }
        if state_id == 25940 {
            return Some(ExposedCopperDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
            });
        }
        return None;
    }
}

