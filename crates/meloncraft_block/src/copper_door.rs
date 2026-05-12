use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopperDoor {
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

impl BlockState for CopperDoor {
    fn to_id(self) -> i32 {
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#open == false { return 25827; }
        if block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left { return 25823; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#open == false && block_state.r#hinge == Hinge::Right { return 25883; }
        if block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#powered == true { return 25853; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower { return 25851; }
        if block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#half == Half::Lower { return 25830; }
        if block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Upper { return 25874; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left { return 25869; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#open == false { return 25852; }
        if block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::South && block_state.r#open == true { return 25837; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#powered == false { return 25826; }
        if block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left && block_state.r#open == true { return 25822; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#half == Half::Lower { return 25879; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper { return 25840; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::North { return 25831; }
        if block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#hinge == Hinge::Left { return 25870; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North { return 25832; }
        if block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#powered == true { return 25839; }
        if block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left { return 25862; }
        if block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East { return 25873; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#open == true { return 25845; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::East { return 25878; }
        if block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South { return 25850; }
        if block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#powered == false { return 25864; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#hinge == Hinge::Right { return 25858; }
        if block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#half == Half::Upper { return 25859; }
        if block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#hinge == Hinge::Left { return 25880; }
        if block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#facing == Facing::East { return 25882; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#open == false { return 25868; }
        if block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::West { return 25863; }
        if block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right { return 25881; }
        if block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#powered == false { return 25844; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == false { return 25846; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::South { return 25841; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#powered == true { return 25825; }
        if block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#half == Half::Lower { return 25847; }
        if block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#hinge == Hinge::Right { return 25842; }
        if block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::West { return 25860; }
        if block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#hinge == Hinge::Left { return 25861; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#powered == true { return 25835; }
        if block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#open == true { return 25834; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#open == true { return 25833; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#powered == true && block_state.r#hinge == Hinge::Right { return 25865; }
        if block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#open == true { return 25866; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right { return 25876; }
        if block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left { return 25877; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#half == Half::Upper { return 25828; }
        if block_state.r#powered == true && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left { return 25855; }
        if block_state.r#powered == true && block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#facing == Facing::East { return 25871; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#half == Half::Lower { return 25867; }
        if block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#powered == false { return 25884; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#powered == false { return 25872; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#powered == true && block_state.r#hinge == Hinge::Left { return 25829; }
        if block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper { return 25875; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::South { return 25848; }
        if block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#facing == Facing::South { return 25849; }
        if block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#facing == Facing::South { return 25838; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#open == false && block_state.r#hinge == Hinge::Right { return 25836; }
        if block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#powered == false { return 25854; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#open == false && block_state.r#hinge == Hinge::Left { return 25824; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::North { return 25821; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#facing == Facing::South && block_state.r#open == false { return 25843; }
        if block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#facing == Facing::West { return 25857; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#hinge == Hinge::Left { return 25856; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25827 {
            return Some(CopperDoor {
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 25823 {
            return Some(CopperDoor {
                r#half: Half::Upper,
                r#open: false,
                r#powered: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25883 {
            return Some(CopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25853 {
            return Some(CopperDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 25851 {
            return Some(CopperDoor {
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Lower,
            });
        }
        if state_id == 25830 {
            return Some(CopperDoor {
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#open: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 25874 {
            return Some(CopperDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 25869 {
            return Some(CopperDoor {
                r#powered: true,
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25852 {
            return Some(CopperDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 25837 {
            return Some(CopperDoor {
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 25826 {
            return Some(CopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 25822 {
            return Some(CopperDoor {
                r#half: Half::Upper,
                r#powered: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 25879 {
            return Some(CopperDoor {
                r#facing: Facing::East,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#open: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 25840 {
            return Some(CopperDoor {
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Upper,
            });
        }
        if state_id == 25831 {
            return Some(CopperDoor {
                r#powered: true,
                r#hinge: Hinge::Left,
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::North,
            });
        }
        if state_id == 25870 {
            return Some(CopperDoor {
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#powered: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25832 {
            return Some(CopperDoor {
                r#open: false,
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 25839 {
            return Some(CopperDoor {
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 25862 {
            return Some(CopperDoor {
                r#open: true,
                r#facing: Facing::West,
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25873 {
            return Some(CopperDoor {
                r#powered: true,
                r#half: Half::Upper,
                r#open: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 25845 {
            return Some(CopperDoor {
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        if state_id == 25878 {
            return Some(CopperDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::East,
            });
        }
        if state_id == 25850 {
            return Some(CopperDoor {
                r#half: Half::Lower,
                r#powered: false,
                r#open: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
            });
        }
        if state_id == 25864 {
            return Some(CopperDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 25858 {
            return Some(CopperDoor {
                r#powered: false,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25859 {
            return Some(CopperDoor {
                r#facing: Facing::West,
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 25880 {
            return Some(CopperDoor {
                r#half: Half::Lower,
                r#powered: false,
                r#facing: Facing::East,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25882 {
            return Some(CopperDoor {
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 25868 {
            return Some(CopperDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#powered: false,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 25863 {
            return Some(CopperDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: false,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 25881 {
            return Some(CopperDoor {
                r#open: true,
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25844 {
            return Some(CopperDoor {
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#powered: false,
            });
        }
        if state_id == 25846 {
            return Some(CopperDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 25841 {
            return Some(CopperDoor {
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Upper,
                r#facing: Facing::South,
            });
        }
        if state_id == 25825 {
            return Some(CopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 25847 {
            return Some(CopperDoor {
                r#facing: Facing::South,
                r#open: false,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 25842 {
            return Some(CopperDoor {
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Upper,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25860 {
            return Some(CopperDoor {
                r#half: Half::Upper,
                r#powered: false,
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
            });
        }
        if state_id == 25861 {
            return Some(CopperDoor {
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#open: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25835 {
            return Some(CopperDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: false,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 25834 {
            return Some(CopperDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 25833 {
            return Some(CopperDoor {
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 25865 {
            return Some(CopperDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25866 {
            return Some(CopperDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 25876 {
            return Some(CopperDoor {
                r#powered: false,
                r#open: false,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25877 {
            return Some(CopperDoor {
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25828 {
            return Some(CopperDoor {
                r#facing: Facing::North,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 25855 {
            return Some(CopperDoor {
                r#powered: true,
                r#open: false,
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25871 {
            return Some(CopperDoor {
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#facing: Facing::East,
            });
        }
        if state_id == 25867 {
            return Some(CopperDoor {
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 25884 {
            return Some(CopperDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 25872 {
            return Some(CopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 25829 {
            return Some(CopperDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25875 {
            return Some(CopperDoor {
                r#open: false,
                r#facing: Facing::East,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
            });
        }
        if state_id == 25848 {
            return Some(CopperDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 25849 {
            return Some(CopperDoor {
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::South,
            });
        }
        if state_id == 25838 {
            return Some(CopperDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 25836 {
            return Some(CopperDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#powered: false,
                r#open: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25854 {
            return Some(CopperDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 25824 {
            return Some(CopperDoor {
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#powered: false,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25821 {
            return Some(CopperDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#open: true,
                r#half: Half::Upper,
                r#facing: Facing::North,
            });
        }
        if state_id == 25843 {
            return Some(CopperDoor {
                r#powered: true,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 25857 {
            return Some(CopperDoor {
                r#open: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 25856 {
            return Some(CopperDoor {
                r#powered: false,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        return None;
    }
}

