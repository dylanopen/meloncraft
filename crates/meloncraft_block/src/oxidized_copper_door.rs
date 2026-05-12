use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OxidizedCopperDoor {
    pub r#half: Half,
    pub r#hinge: Hinge,
    pub r#facing: Facing,
    pub open: bool,
    pub powered: bool,
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

impl BlockState for OxidizedCopperDoor {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == false { return 25970; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#open == false { return 25999; }
        if block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::North { return 25962; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::North && block_state.r#open == false { return 25951; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == true { return 25991; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == false && block_state.r#hinge == Hinge::Right { return 25964; }
        if block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Left { return 25967; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#powered == true { return 26009; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#powered == true { return 25979; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == false { return 25974; }
        if block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#open == false { return 25980; }
        if block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#facing == Facing::North { return 25956; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#open == true && block_state.r#hinge == Hinge::Right { return 25986; }
        if block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#half == Half::Upper { return 25968; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#facing == Facing::North { return 25952; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#powered == false { return 25972; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#open == false { return 26004; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#facing == Facing::West { return 25988; }
        if block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#open == false && block_state.r#half == Half::Lower { return 26011; }
        if block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::West { return 25992; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#open == true { return 25954; }
        if block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower { return 25977; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#open == false { return 26012; }
        if block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East { return 26003; }
        if block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper { return 25969; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#open == true { return 25957; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#open == true { return 25978; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#open == false { return 25959; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#half == Half::Upper { return 25984; }
        if block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#facing == Facing::West { return 25993; }
        if block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::South && block_state.r#powered == true { return 25975; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#hinge == Hinge::Left { return 25973; }
        if block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#powered == true { return 25995; }
        if block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == false && block_state.r#half == Half::Upper { return 26000; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == true { return 25981; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#facing == Facing::East { return 26005; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == false { return 26010; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left { return 25997; }
        if block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#half == Half::Lower { return 25989; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == false && block_state.r#hinge == Hinge::Right { return 25996; }
        if block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#powered == true { return 26007; }
        if block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right { return 25985; }
        if block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#powered == false { return 25994; }
        if block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#hinge == Hinge::Left { return 25998; }
        if block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#open == true { return 26001; }
        if block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#hinge == Hinge::Left { return 25950; }
        if block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right { return 25953; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#open == false { return 25976; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == false { return 25960; }
        if block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#facing == Facing::North { return 25963; }
        if block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#powered == false { return 26006; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#facing == Facing::West { return 25987; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#facing == Facing::North { return 25949; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#open == false && block_state.r#hinge == Hinge::Right { return 25971; }
        if block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left { return 25966; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#half == Half::Lower { return 25958; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#open == false { return 26008; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower { return 25961; }
        if block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#hinge == Hinge::Right { return 25955; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#open == true { return 25982; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == true { return 25965; }
        if block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#open == true { return 26002; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left { return 25983; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#open == true { return 25990; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25970 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 25999 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 25962 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Lower,
                r#powered: false,
                r#open: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 25951 {
            return Some(OxidizedCopperDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 25991 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 25964 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25967 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Upper,
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26009 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: true,
            });
        }
        if state_id == 25979 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 25974 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 25980 {
            return Some(OxidizedCopperDoor {
                r#powered: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 25956 {
            return Some(OxidizedCopperDoor {
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Upper,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 25986 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#powered: false,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25968 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::South,
                r#open: false,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 25952 {
            return Some(OxidizedCopperDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 25972 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26004 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 25988 {
            return Some(OxidizedCopperDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::West,
            });
        }
        if state_id == 26011 {
            return Some(OxidizedCopperDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#powered: true,
                r#open: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 25992 {
            return Some(OxidizedCopperDoor {
                r#powered: false,
                r#hinge: Hinge::Left,
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::West,
            });
        }
        if state_id == 25954 {
            return Some(OxidizedCopperDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 25977 {
            return Some(OxidizedCopperDoor {
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Lower,
            });
        }
        if state_id == 26012 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 26003 {
            return Some(OxidizedCopperDoor {
                r#open: false,
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 25969 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
            });
        }
        if state_id == 25957 {
            return Some(OxidizedCopperDoor {
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 25978 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 25959 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 25984 {
            return Some(OxidizedCopperDoor {
                r#hinge: Hinge::Left,
                r#powered: false,
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 25993 {
            return Some(OxidizedCopperDoor {
                r#open: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 25975 {
            return Some(OxidizedCopperDoor {
                r#open: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 25973 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::South,
                r#powered: true,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25995 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Lower,
                r#open: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#powered: true,
            });
        }
        if state_id == 26000 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 25981 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26005 {
            return Some(OxidizedCopperDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 26010 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 25997 {
            return Some(OxidizedCopperDoor {
                r#open: true,
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25989 {
            return Some(OxidizedCopperDoor {
                r#open: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 25996 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26007 {
            return Some(OxidizedCopperDoor {
                r#open: false,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 25985 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Upper,
                r#powered: true,
                r#open: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25994 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 25998 {
            return Some(OxidizedCopperDoor {
                r#open: true,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#powered: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26001 {
            return Some(OxidizedCopperDoor {
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 25950 {
            return Some(OxidizedCopperDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#open: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25953 {
            return Some(OxidizedCopperDoor {
                r#open: true,
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25976 {
            return Some(OxidizedCopperDoor {
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 25960 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 25963 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Lower,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 26006 {
            return Some(OxidizedCopperDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 25987 {
            return Some(OxidizedCopperDoor {
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#facing: Facing::West,
            });
        }
        if state_id == 25949 {
            return Some(OxidizedCopperDoor {
                r#powered: true,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 25971 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25966 {
            return Some(OxidizedCopperDoor {
                r#open: true,
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25958 {
            return Some(OxidizedCopperDoor {
                r#powered: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 26008 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 25961 {
            return Some(OxidizedCopperDoor {
                r#open: true,
                r#powered: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
            });
        }
        if state_id == 25955 {
            return Some(OxidizedCopperDoor {
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25982 {
            return Some(OxidizedCopperDoor {
                r#hinge: Hinge::Left,
                r#powered: false,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 25965 {
            return Some(OxidizedCopperDoor {
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 26002 {
            return Some(OxidizedCopperDoor {
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 25983 {
            return Some(OxidizedCopperDoor {
                r#open: false,
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25990 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: true,
            });
        }
        return None;
    }
}

