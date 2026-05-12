use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchDoor {
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

impl BlockState for BirchDoor {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#half == Half::Upper { return 13942; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left { return 13979; }
        if block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#hinge == Hinge::Right { return 13950; }
        if block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#open == false { return 13933; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right { return 13935; }
        if block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#powered == true { return 13938; }
        if block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#powered == true { return 13954; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == false { return 13955; }
        if block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::North { return 13926; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#powered == false { return 13963; }
        if block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right && block_state.r#powered == true { return 13974; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#hinge == Hinge::Right { return 13983; }
        if block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right && block_state.r#powered == true { return 13934; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#open == false && block_state.r#hinge == Hinge::Left { return 13932; }
        if block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#half == Half::Upper { return 13925; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#hinge == Hinge::Right { return 13927; }
        if block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#powered == false { return 13967; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#powered == true { return 13940; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#powered == false { return 13945; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == false { return 13949; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#powered == false { return 13969; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#half == Half::Upper { return 13977; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower { return 13962; }
        if block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower { return 13966; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper { return 13970; }
        if block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper { return 13943; }
        if block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#facing == Facing::North { return 13929; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#powered == true { return 13936; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#open == false { return 13985; }
        if block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left { return 13965; }
        if block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#open == true { return 13923; }
        if block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper { return 13924; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper { return 13928; }
        if block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#hinge == Hinge::Right { return 13953; }
        if block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#half == Half::Lower { return 13946; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#open == true { return 13930; }
        if block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#powered == true { return 13944; }
        if block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left { return 13922; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::East { return 13972; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == true { return 13948; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#powered == true { return 13968; }
        if block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East { return 13973; }
        if block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right && block_state.r#open == true { return 13951; }
        if block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper { return 13975; }
        if block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#powered == false { return 13947; }
        if block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#facing == Facing::West { return 13958; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#open == false && block_state.r#hinge == Hinge::Right { return 13976; }
        if block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::North && block_state.r#open == false { return 13937; }
        if block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#open == true && block_state.r#half == Half::Lower { return 13982; }
        if block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#hinge == Hinge::Left { return 13957; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#powered == false { return 13961; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::East { return 13978; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left { return 13941; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East && block_state.r#half == Half::Lower { return 13984; }
        if block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#powered == false { return 13931; }
        if block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#powered == true { return 13980; }
        if block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right { return 13960; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#open == true { return 13939; }
        if block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#powered == false { return 13971; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left { return 13956; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#half == Half::Lower { return 13981; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#open == false { return 13952; }
        if block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right { return 13959; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#open == false && block_state.r#hinge == Hinge::Left { return 13964; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13942 {
            return Some(BirchDoor {
                r#facing: Facing::South,
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 13979 {
            return Some(BirchDoor {
                r#open: true,
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 13950 {
            return Some(BirchDoor {
                r#open: true,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 13933 {
            return Some(BirchDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 13935 {
            return Some(BirchDoor {
                r#powered: false,
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 13938 {
            return Some(BirchDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 13954 {
            return Some(BirchDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 13955 {
            return Some(BirchDoor {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 13926 {
            return Some(BirchDoor {
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::North,
            });
        }
        if state_id == 13963 {
            return Some(BirchDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 13974 {
            return Some(BirchDoor {
                r#open: true,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#powered: true,
            });
        }
        if state_id == 13983 {
            return Some(BirchDoor {
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 13934 {
            return Some(BirchDoor {
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#powered: true,
            });
        }
        if state_id == 13932 {
            return Some(BirchDoor {
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 13925 {
            return Some(BirchDoor {
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#open: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 13927 {
            return Some(BirchDoor {
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Upper,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 13967 {
            return Some(BirchDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 13940 {
            return Some(BirchDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 13945 {
            return Some(BirchDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#powered: false,
            });
        }
        if state_id == 13949 {
            return Some(BirchDoor {
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 13969 {
            return Some(BirchDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: false,
            });
        }
        if state_id == 13977 {
            return Some(BirchDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 13962 {
            return Some(BirchDoor {
                r#facing: Facing::West,
                r#powered: true,
                r#open: true,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
            });
        }
        if state_id == 13966 {
            return Some(BirchDoor {
                r#powered: true,
                r#open: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
            });
        }
        if state_id == 13970 {
            return Some(BirchDoor {
                r#facing: Facing::East,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
            });
        }
        if state_id == 13943 {
            return Some(BirchDoor {
                r#facing: Facing::South,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
            });
        }
        if state_id == 13929 {
            return Some(BirchDoor {
                r#half: Half::Upper,
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 13936 {
            return Some(BirchDoor {
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 13985 {
            return Some(BirchDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 13965 {
            return Some(BirchDoor {
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 13923 {
            return Some(BirchDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 13924 {
            return Some(BirchDoor {
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
            });
        }
        if state_id == 13928 {
            return Some(BirchDoor {
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Upper,
            });
        }
        if state_id == 13953 {
            return Some(BirchDoor {
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 13946 {
            return Some(BirchDoor {
                r#open: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 13930 {
            return Some(BirchDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: true,
            });
        }
        if state_id == 13944 {
            return Some(BirchDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 13922 {
            return Some(BirchDoor {
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 13972 {
            return Some(BirchDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 13948 {
            return Some(BirchDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 13968 {
            return Some(BirchDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: true,
            });
        }
        if state_id == 13973 {
            return Some(BirchDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#open: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 13951 {
            return Some(BirchDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 13975 {
            return Some(BirchDoor {
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
            });
        }
        if state_id == 13947 {
            return Some(BirchDoor {
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 13958 {
            return Some(BirchDoor {
                r#open: true,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 13976 {
            return Some(BirchDoor {
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 13937 {
            return Some(BirchDoor {
                r#half: Half::Lower,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 13982 {
            return Some(BirchDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#open: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 13957 {
            return Some(BirchDoor {
                r#open: false,
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#powered: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 13961 {
            return Some(BirchDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 13978 {
            return Some(BirchDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#open: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 13941 {
            return Some(BirchDoor {
                r#powered: false,
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 13984 {
            return Some(BirchDoor {
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#half: Half::Lower,
            });
        }
        if state_id == 13931 {
            return Some(BirchDoor {
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 13980 {
            return Some(BirchDoor {
                r#open: false,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 13960 {
            return Some(BirchDoor {
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 13939 {
            return Some(BirchDoor {
                r#powered: false,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 13971 {
            return Some(BirchDoor {
                r#open: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 13956 {
            return Some(BirchDoor {
                r#open: false,
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 13981 {
            return Some(BirchDoor {
                r#hinge: Hinge::Left,
                r#open: false,
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 13952 {
            return Some(BirchDoor {
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 13959 {
            return Some(BirchDoor {
                r#half: Half::Upper,
                r#open: true,
                r#powered: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 13964 {
            return Some(BirchDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        return None;
    }
}

