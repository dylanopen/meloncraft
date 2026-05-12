use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpruceDoor {
    pub open: bool,
    pub powered: bool,
    pub r#half: Half,
    pub r#facing: Facing,
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

impl BlockState for SpruceDoor {
    fn to_id(self) -> i32 {
        if block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper && block_state.r#powered == false { return 13863; }
        if block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == false { return 13869; }
        if block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#open == true { return 13895; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#open == false && block_state.r#facing == Facing::West { return 13904; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#half == Half::Lower { return 13885; }
        if block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#facing == Facing::East { return 13907; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East { return 13920; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == false { return 13883; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#facing == Facing::North { return 13870; }
        if block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#open == false { return 13880; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#open == false { return 13905; }
        if block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#open == true { return 13914; }
        if block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#open == true { return 13890; }
        if block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#open == true { return 13915; }
        if block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#powered == false { return 13893; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::South { return 13876; }
        if block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#powered == true { return 13874; }
        if block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East && block_state.r#powered == true { return 13912; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == true && block_state.r#hinge == Hinge::Right { return 13888; }
        if block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#facing == Facing::East { return 13919; }
        if block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#facing == Facing::West { return 13903; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == true { return 13875; }
        if block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#open == true { return 13910; }
        if block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#powered == true { return 13898; }
        if block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#powered == false { return 13909; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#open == false { return 13900; }
        if block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right { return 13872; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#open == false { return 13881; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#powered == false { return 13921; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left { return 13877; }
        if block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#half == Half::Lower { return 13867; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#facing == Facing::West { return 13901; }
        if block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#open == false { return 13916; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#facing == Facing::South { return 13887; }
        if block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East { return 13918; }
        if block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right && block_state.r#powered == false { return 13911; }
        if block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right { return 13913; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#open == false { return 13865; }
        if block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right { return 13894; }
        if block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#open == true { return 13871; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#powered == true { return 13866; }
        if block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#facing == Facing::West && block_state.r#open == false { return 13897; }
        if block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Upper { return 13862; }
        if block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right && block_state.r#open == true { return 13878; }
        if block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#hinge == Hinge::Right { return 13896; }
        if block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#powered == true { return 13908; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#powered == true { return 13886; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Lower { return 13868; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#facing == Facing::North { return 13860; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#open == true { return 13858; }
        if block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#hinge == Hinge::Left { return 13884; }
        if block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper { return 13859; }
        if block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#powered == false { return 13917; }
        if block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::North { return 13873; }
        if block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#hinge == Hinge::Left { return 13899; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#facing == Facing::North { return 13861; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper { return 13864; }
        if block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#half == Half::Upper { return 13879; }
        if block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == true { return 13892; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper { return 13891; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right { return 13889; }
        if block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#powered == true { return 13902; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#open == true { return 13906; }
        if block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#open == true { return 13882; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13863 {
            return Some(SpruceDoor {
                r#open: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#powered: false,
            });
        }
        if state_id == 13869 {
            return Some(SpruceDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 13895 {
            return Some(SpruceDoor {
                r#powered: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 13904 {
            return Some(SpruceDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#powered: true,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 13885 {
            return Some(SpruceDoor {
                r#facing: Facing::South,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#open: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 13907 {
            return Some(SpruceDoor {
                r#half: Half::Upper,
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 13920 {
            return Some(SpruceDoor {
                r#open: false,
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 13883 {
            return Some(SpruceDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 13870 {
            return Some(SpruceDoor {
                r#powered: true,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 13880 {
            return Some(SpruceDoor {
                r#hinge: Hinge::Right,
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#open: false,
            });
        }
        if state_id == 13905 {
            return Some(SpruceDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 13914 {
            return Some(SpruceDoor {
                r#half: Half::Lower,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 13890 {
            return Some(SpruceDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 13915 {
            return Some(SpruceDoor {
                r#half: Half::Lower,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 13893 {
            return Some(SpruceDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 13876 {
            return Some(SpruceDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::South,
            });
        }
        if state_id == 13874 {
            return Some(SpruceDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 13912 {
            return Some(SpruceDoor {
                r#open: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 13888 {
            return Some(SpruceDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 13919 {
            return Some(SpruceDoor {
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 13903 {
            return Some(SpruceDoor {
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Lower,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 13875 {
            return Some(SpruceDoor {
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 13910 {
            return Some(SpruceDoor {
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 13898 {
            return Some(SpruceDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 13909 {
            return Some(SpruceDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 13900 {
            return Some(SpruceDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 13872 {
            return Some(SpruceDoor {
                r#half: Half::Lower,
                r#open: false,
                r#powered: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 13881 {
            return Some(SpruceDoor {
                r#facing: Facing::South,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#open: false,
            });
        }
        if state_id == 13921 {
            return Some(SpruceDoor {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 13877 {
            return Some(SpruceDoor {
                r#facing: Facing::South,
                r#powered: false,
                r#open: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 13867 {
            return Some(SpruceDoor {
                r#open: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 13901 {
            return Some(SpruceDoor {
                r#hinge: Hinge::Left,
                r#open: false,
                r#half: Half::Lower,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 13916 {
            return Some(SpruceDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 13887 {
            return Some(SpruceDoor {
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#facing: Facing::South,
            });
        }
        if state_id == 13918 {
            return Some(SpruceDoor {
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 13911 {
            return Some(SpruceDoor {
                r#half: Half::Upper,
                r#open: true,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#powered: false,
            });
        }
        if state_id == 13913 {
            return Some(SpruceDoor {
                r#open: false,
                r#half: Half::Upper,
                r#powered: false,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 13865 {
            return Some(SpruceDoor {
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 13894 {
            return Some(SpruceDoor {
                r#powered: true,
                r#open: true,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 13871 {
            return Some(SpruceDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Lower,
                r#open: true,
            });
        }
        if state_id == 13866 {
            return Some(SpruceDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 13897 {
            return Some(SpruceDoor {
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 13862 {
            return Some(SpruceDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 13878 {
            return Some(SpruceDoor {
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 13896 {
            return Some(SpruceDoor {
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 13908 {
            return Some(SpruceDoor {
                r#open: false,
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 13886 {
            return Some(SpruceDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 13868 {
            return Some(SpruceDoor {
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Lower,
            });
        }
        if state_id == 13860 {
            return Some(SpruceDoor {
                r#powered: true,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 13858 {
            return Some(SpruceDoor {
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 13884 {
            return Some(SpruceDoor {
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 13859 {
            return Some(SpruceDoor {
                r#open: true,
                r#facing: Facing::North,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
            });
        }
        if state_id == 13917 {
            return Some(SpruceDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 13873 {
            return Some(SpruceDoor {
                r#powered: false,
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::North,
            });
        }
        if state_id == 13899 {
            return Some(SpruceDoor {
                r#open: true,
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#powered: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 13861 {
            return Some(SpruceDoor {
                r#powered: false,
                r#open: false,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#facing: Facing::North,
            });
        }
        if state_id == 13864 {
            return Some(SpruceDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Upper,
            });
        }
        if state_id == 13879 {
            return Some(SpruceDoor {
                r#open: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 13892 {
            return Some(SpruceDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 13891 {
            return Some(SpruceDoor {
                r#powered: false,
                r#facing: Facing::West,
                r#open: true,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
            });
        }
        if state_id == 13889 {
            return Some(SpruceDoor {
                r#open: false,
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 13902 {
            return Some(SpruceDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 13906 {
            return Some(SpruceDoor {
                r#powered: true,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 13882 {
            return Some(SpruceDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Lower,
                r#open: true,
            });
        }
        return None;
    }
}

