use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OxidizedCopperDoor {
    pub r#facing: Facing,
    pub r#hinge: Hinge,
    pub powered: bool,
    pub open: bool,
    pub r#half: Half,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Upper,
    Lower,
}

impl BlockState for OxidizedCopperDoor {
    fn to_id(&self) -> i32 {
        if self.r#hinge == Hinge::Left && self.r#half == Half::Lower && self.r#facing == Facing::West && self.r#open == false && self.r#powered == false { return 25992; }
        if self.r#facing == Facing::North && self.r#half == Half::Lower && self.r#open == true && self.r#powered == false && self.r#hinge == Hinge::Right { return 25962; }
        if self.r#hinge == Hinge::Left && self.r#facing == Facing::South && self.r#open == true && self.r#powered == true && self.r#half == Half::Lower { return 25973; }
        if self.r#hinge == Hinge::Left && self.r#powered == true && self.r#half == Half::Upper && self.r#facing == Facing::North && self.r#open == true { return 25949; }
        if self.r#facing == Facing::South && self.r#half == Half::Lower && self.r#hinge == Hinge::Right && self.r#open == false && self.r#powered == false { return 25980; }
        if self.r#facing == Facing::South && self.r#open == false && self.r#hinge == Hinge::Right && self.r#half == Half::Upper && self.r#powered == false { return 25972; }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#hinge == Hinge::Right && self.r#half == Half::Lower && self.r#open == false { return 26012; }
        if self.r#half == Half::Lower && self.r#open == false && self.r#hinge == Hinge::Right && self.r#powered == true && self.r#facing == Facing::South { return 25979; }
        if self.r#powered == true && self.r#facing == Facing::North && self.r#half == Half::Upper && self.r#open == false && self.r#hinge == Hinge::Left { return 25951; }
        if self.r#facing == Facing::West && self.r#open == false && self.r#powered == false && self.r#hinge == Hinge::Right && self.r#half == Half::Lower { return 25996; }
        if self.r#open == true && self.r#hinge == Hinge::Right && self.r#facing == Facing::South && self.r#powered == false && self.r#half == Half::Lower { return 25978; }
        if self.r#powered == true && self.r#hinge == Hinge::Left && self.r#half == Half::Lower && self.r#facing == Facing::North && self.r#open == false { return 25959; }
        if self.r#facing == Facing::West && self.r#hinge == Hinge::Right && self.r#half == Half::Upper && self.r#powered == true && self.r#open == true { return 25985; }
        if self.r#powered == false && self.r#hinge == Hinge::Left && self.r#open == false && self.r#half == Half::Upper && self.r#facing == Facing::East { return 26000; }
        if self.r#powered == true && self.r#hinge == Hinge::Right && self.r#facing == Facing::East && self.r#open == false && self.r#half == Half::Upper { return 26003; }
        if self.r#hinge == Hinge::Right && self.r#powered == true && self.r#half == Half::Lower && self.r#open == false && self.r#facing == Facing::West { return 25995; }
        if self.r#open == true && self.r#half == Half::Lower && self.r#powered == false && self.r#hinge == Hinge::Left && self.r#facing == Facing::South { return 25974; }
        if self.r#facing == Facing::East && self.r#open == true && self.r#powered == false && self.r#hinge == Hinge::Left && self.r#half == Half::Lower { return 26006; }
        if self.r#powered == false && self.r#half == Half::Upper && self.r#hinge == Hinge::Left && self.r#facing == Facing::South && self.r#open == true { return 25966; }
        if self.r#open == false && self.r#half == Half::Lower && self.r#facing == Facing::South && self.r#hinge == Hinge::Left && self.r#powered == false { return 25976; }
        if self.r#powered == true && self.r#open == true && self.r#half == Half::Upper && self.r#facing == Facing::East && self.r#hinge == Hinge::Left { return 25997; }
        if self.r#powered == false && self.r#hinge == Hinge::Right && self.r#half == Half::Upper && self.r#facing == Facing::West && self.r#open == true { return 25986; }
        if self.r#open == false && self.r#powered == false && self.r#half == Half::Lower && self.r#facing == Facing::North && self.r#hinge == Hinge::Right { return 25964; }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#half == Half::Upper && self.r#open == true && self.r#hinge == Hinge::Right { return 26002; }
        if self.r#half == Half::Lower && self.r#hinge == Hinge::Right && self.r#facing == Facing::North && self.r#open == true && self.r#powered == true { return 25961; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Right && self.r#open == false && self.r#powered == false && self.r#facing == Facing::West { return 25988; }
        if self.r#facing == Facing::South && self.r#open == true && self.r#half == Half::Upper && self.r#powered == false && self.r#hinge == Hinge::Right { return 25970; }
        if self.r#powered == true && self.r#hinge == Hinge::Left && self.r#facing == Facing::West && self.r#half == Half::Upper && self.r#open == false { return 25983; }
        if self.r#hinge == Hinge::Left && self.r#open == true && self.r#facing == Facing::North && self.r#half == Half::Upper && self.r#powered == false { return 25950; }
        if self.r#hinge == Hinge::Right && self.r#half == Half::Lower && self.r#facing == Facing::West && self.r#open == true && self.r#powered == true { return 25993; }
        if self.r#facing == Facing::South && self.r#open == false && self.r#hinge == Hinge::Left && self.r#half == Half::Lower && self.r#powered == true { return 25975; }
        if self.r#powered == false && self.r#hinge == Hinge::Left && self.r#facing == Facing::North && self.r#half == Half::Upper && self.r#open == false { return 25952; }
        if self.r#hinge == Hinge::Left && self.r#open == true && self.r#powered == false && self.r#facing == Facing::East && self.r#half == Half::Upper { return 25998; }
        if self.r#powered == true && self.r#half == Half::Upper && self.r#facing == Facing::North && self.r#hinge == Hinge::Right && self.r#open == false { return 25955; }
        if self.r#powered == false && self.r#facing == Facing::North && self.r#hinge == Hinge::Left && self.r#half == Half::Lower && self.r#open == false { return 25960; }
        if self.r#open == true && self.r#half == Half::Lower && self.r#hinge == Hinge::Right && self.r#facing == Facing::East && self.r#powered == true { return 26009; }
        if self.r#hinge == Hinge::Left && self.r#half == Half::Upper && self.r#powered == false && self.r#facing == Facing::West && self.r#open == false { return 25984; }
        if self.r#powered == true && self.r#open == true && self.r#half == Half::Lower && self.r#facing == Facing::East && self.r#hinge == Hinge::Left { return 26005; }
        if self.r#facing == Facing::East && self.r#open == true && self.r#half == Half::Lower && self.r#hinge == Hinge::Right && self.r#powered == false { return 26010; }
        if self.r#facing == Facing::West && self.r#hinge == Hinge::Left && self.r#half == Half::Upper && self.r#open == true && self.r#powered == true { return 25981; }
        if self.r#powered == true && self.r#half == Half::Upper && self.r#open == true && self.r#facing == Facing::South && self.r#hinge == Hinge::Right { return 25969; }
        if self.r#open == true && self.r#powered == true && self.r#hinge == Hinge::Right && self.r#facing == Facing::South && self.r#half == Half::Lower { return 25977; }
        if self.r#open == true && self.r#hinge == Hinge::Right && self.r#powered == true && self.r#facing == Facing::East && self.r#half == Half::Upper { return 26001; }
        if self.r#facing == Facing::West && self.r#hinge == Hinge::Left && self.r#open == true && self.r#half == Half::Upper && self.r#powered == false { return 25982; }
        if self.r#half == Half::Upper && self.r#open == false && self.r#facing == Facing::South && self.r#powered == true && self.r#hinge == Hinge::Left { return 25967; }
        if self.r#open == false && self.r#powered == false && self.r#hinge == Hinge::Left && self.r#facing == Facing::East && self.r#half == Half::Lower { return 26008; }
        if self.r#powered == true && self.r#hinge == Hinge::Right && self.r#facing == Facing::East && self.r#half == Half::Lower && self.r#open == false { return 26011; }
        if self.r#half == Half::Lower && self.r#open == true && self.r#powered == true && self.r#hinge == Hinge::Left && self.r#facing == Facing::North { return 25957; }
        if self.r#half == Half::Lower && self.r#open == false && self.r#powered == true && self.r#facing == Facing::North && self.r#hinge == Hinge::Right { return 25963; }
        if self.r#half == Half::Upper && self.r#hinge == Hinge::Right && self.r#facing == Facing::North && self.r#powered == false && self.r#open == false { return 25956; }
        if self.r#open == false && self.r#hinge == Hinge::Left && self.r#facing == Facing::South && self.r#half == Half::Upper && self.r#powered == false { return 25968; }
        if self.r#powered == true && self.r#hinge == Hinge::Right && self.r#open == true && self.r#facing == Facing::North && self.r#half == Half::Upper { return 25953; }
        if self.r#facing == Facing::West && self.r#half == Half::Lower && self.r#open == true && self.r#hinge == Hinge::Left && self.r#powered == true { return 25989; }
        if self.r#powered == false && self.r#half == Half::Upper && self.r#facing == Facing::East && self.r#open == false && self.r#hinge == Hinge::Right { return 26004; }
        if self.r#open == true && self.r#hinge == Hinge::Left && self.r#facing == Facing::South && self.r#powered == true && self.r#half == Half::Upper { return 25965; }
        if self.r#hinge == Hinge::Right && self.r#half == Half::Upper && self.r#facing == Facing::North && self.r#open == true && self.r#powered == false { return 25954; }
        if self.r#half == Half::Lower && self.r#open == true && self.r#hinge == Hinge::Right && self.r#facing == Facing::West && self.r#powered == false { return 25994; }
        if self.r#half == Half::Lower && self.r#facing == Facing::East && self.r#powered == true && self.r#open == false && self.r#hinge == Hinge::Left { return 26007; }
        if self.r#powered == true && self.r#open == false && self.r#hinge == Hinge::Left && self.r#facing == Facing::East && self.r#half == Half::Upper { return 25999; }
        if self.r#facing == Facing::North && self.r#hinge == Hinge::Left && self.r#half == Half::Lower && self.r#open == true && self.r#powered == false { return 25958; }
        if self.r#hinge == Hinge::Left && self.r#half == Half::Lower && self.r#powered == false && self.r#facing == Facing::West && self.r#open == true { return 25990; }
        if self.r#hinge == Hinge::Right && self.r#facing == Facing::West && self.r#open == false && self.r#half == Half::Upper && self.r#powered == true { return 25987; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#hinge == Hinge::Left && self.r#half == Half::Lower && self.r#open == false { return 25991; }
        if self.r#hinge == Hinge::Right && self.r#open == false && self.r#facing == Facing::South && self.r#half == Half::Upper && self.r#powered == true { return 25971; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25992 {
            return Some(OxidizedCopperDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 25962 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25973 {
            return Some(OxidizedCopperDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 25949 {
            return Some(OxidizedCopperDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 25980 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 25972 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::South,
                r#open: false,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#powered: false,
            });
        }
        if state_id == 26012 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::East,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 25979 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Lower,
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 25951 {
            return Some(OxidizedCopperDoor {
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25996 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
            });
        }
        if state_id == 25978 {
            return Some(OxidizedCopperDoor {
                r#open: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 25959 {
            return Some(OxidizedCopperDoor {
                r#powered: true,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 25985 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 26000 {
            return Some(OxidizedCopperDoor {
                r#powered: false,
                r#hinge: Hinge::Left,
                r#open: false,
                r#half: Half::Upper,
                r#facing: Facing::East,
            });
        }
        if state_id == 26003 {
            return Some(OxidizedCopperDoor {
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 25995 {
            return Some(OxidizedCopperDoor {
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Lower,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 25974 {
            return Some(OxidizedCopperDoor {
                r#open: true,
                r#half: Half::Lower,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
            });
        }
        if state_id == 26006 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::East,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
            });
        }
        if state_id == 25966 {
            return Some(OxidizedCopperDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 25976 {
            return Some(OxidizedCopperDoor {
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 25997 {
            return Some(OxidizedCopperDoor {
                r#powered: true,
                r#open: true,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25986 {
            return Some(OxidizedCopperDoor {
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 25964 {
            return Some(OxidizedCopperDoor {
                r#open: false,
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 26002 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Upper,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25961 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 25988 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 25970 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Upper,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25983 {
            return Some(OxidizedCopperDoor {
                r#powered: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#open: false,
            });
        }
        if state_id == 25950 {
            return Some(OxidizedCopperDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#powered: false,
            });
        }
        if state_id == 25993 {
            return Some(OxidizedCopperDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 25975 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::South,
                r#open: false,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#powered: true,
            });
        }
        if state_id == 25952 {
            return Some(OxidizedCopperDoor {
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#open: false,
            });
        }
        if state_id == 25998 {
            return Some(OxidizedCopperDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Upper,
            });
        }
        if state_id == 25955 {
            return Some(OxidizedCopperDoor {
                r#powered: true,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 25960 {
            return Some(OxidizedCopperDoor {
                r#powered: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 26009 {
            return Some(OxidizedCopperDoor {
                r#open: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 25984 {
            return Some(OxidizedCopperDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#powered: false,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 26005 {
            return Some(OxidizedCopperDoor {
                r#powered: true,
                r#open: true,
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26010 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#powered: false,
            });
        }
        if state_id == 25981 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 25969 {
            return Some(OxidizedCopperDoor {
                r#powered: true,
                r#half: Half::Upper,
                r#open: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25977 {
            return Some(OxidizedCopperDoor {
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#half: Half::Lower,
            });
        }
        if state_id == 26001 {
            return Some(OxidizedCopperDoor {
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Upper,
            });
        }
        if state_id == 25982 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Upper,
                r#powered: false,
            });
        }
        if state_id == 25967 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::South,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 26008 {
            return Some(OxidizedCopperDoor {
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#half: Half::Lower,
            });
        }
        if state_id == 26011 {
            return Some(OxidizedCopperDoor {
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 25957 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 25963 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Lower,
                r#open: false,
                r#powered: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25956 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 25968 {
            return Some(OxidizedCopperDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#powered: false,
            });
        }
        if state_id == 25953 {
            return Some(OxidizedCopperDoor {
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Upper,
            });
        }
        if state_id == 25989 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 26004 {
            return Some(OxidizedCopperDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#open: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25965 {
            return Some(OxidizedCopperDoor {
                r#open: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 25954 {
            return Some(OxidizedCopperDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 25994 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 26007 {
            return Some(OxidizedCopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25999 {
            return Some(OxidizedCopperDoor {
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#half: Half::Upper,
            });
        }
        if state_id == 25958 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 25990 {
            return Some(OxidizedCopperDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#powered: false,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 25987 {
            return Some(OxidizedCopperDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 25991 {
            return Some(OxidizedCopperDoor {
                r#facing: Facing::West,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 25971 {
            return Some(OxidizedCopperDoor {
                r#hinge: Hinge::Right,
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        return None;
    }
}

