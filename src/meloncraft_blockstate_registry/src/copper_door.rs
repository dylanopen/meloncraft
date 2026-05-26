use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopperDoor {
    pub powered: bool,
    pub r#hinge: Hinge,
    pub open: bool,
    pub r#half: Half,
    pub r#facing: Facing,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for CopperDoor {
    fn to_id(&self) -> i32 {
        if self.r#hinge == Hinge::Left
            && self.r#powered == false
            && self.r#facing == Facing::South
            && self.r#open == true
            && self.r#half == Half::Upper
        {
            return 25838;
        }
        if self.r#open == false
            && self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
        {
            return 25859;
        }
        if self.r#half == Half::Lower
            && self.r#open == false
            && self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#hinge == Hinge::Left
        {
            return 25879;
        }
        if self.r#half == Half::Lower
            && self.r#open == true
            && self.r#powered == true
            && self.r#facing == Facing::North
            && self.r#hinge == Hinge::Right
        {
            return 25833;
        }
        if self.r#open == true
            && self.r#powered == false
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Upper
            && self.r#facing == Facing::North
        {
            return 25826;
        }
        if self.r#open == true
            && self.r#facing == Facing::West
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Upper
            && self.r#powered == true
        {
            return 25857;
        }
        if self.r#open == false
            && self.r#facing == Facing::South
            && self.r#powered == true
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Left
        {
            return 25847;
        }
        if self.r#open == false
            && self.r#powered == true
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::East
        {
            return 25871;
        }
        if self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#open == true
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Left
        {
            return 25822;
        }
        if self.r#facing == Facing::East
            && self.r#open == false
            && self.r#hinge == Hinge::Left
            && self.r#powered == false
            && self.r#half == Half::Upper
        {
            return 25872;
        }
        if self.r#open == false
            && self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Lower
        {
            return 25880;
        }
        if self.r#half == Half::Lower
            && self.r#open == false
            && self.r#powered == false
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::West
        {
            return 25864;
        }
        if self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Lower
            && self.r#open == true
            && self.r#hinge == Hinge::Right
        {
            return 25881;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Lower
            && self.r#powered == false
            && self.r#hinge == Hinge::Right
            && self.r#open == true
        {
            return 25850;
        }
        if self.r#open == true
            && self.r#hinge == Hinge::Left
            && self.r#powered == true
            && self.r#half == Half::Lower
            && self.r#facing == Facing::South
        {
            return 25845;
        }
        if self.r#hinge == Hinge::Right
            && self.r#half == Half::Upper
            && self.r#powered == true
            && self.r#open == true
            && self.r#facing == Facing::South
        {
            return 25841;
        }
        if self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#half == Half::Lower
            && self.r#open == false
            && self.r#hinge == Hinge::Right
        {
            return 25867;
        }
        if self.r#hinge == Hinge::Left
            && self.r#open == false
            && self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Upper
        {
            return 25839;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Lower
            && self.r#open == false
            && self.r#hinge == Hinge::Left
            && self.r#powered == false
        {
            return 25832;
        }
        if self.r#open == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
            && self.r#powered == true
        {
            return 25849;
        }
        if self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#half == Half::Lower
            && self.r#open == true
            && self.r#hinge == Hinge::Left
        {
            return 25830;
        }
        if self.r#facing == Facing::North
            && self.r#hinge == Hinge::Left
            && self.r#open == true
            && self.r#half == Half::Upper
            && self.r#powered == true
        {
            return 25821;
        }
        if self.r#open == false
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::East
            && self.r#half == Half::Lower
            && self.r#powered == false
        {
            return 25884;
        }
        if self.r#open == true
            && self.r#powered == true
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::North
            && self.r#half == Half::Upper
        {
            return 25825;
        }
        if self.r#half == Half::Lower
            && self.r#facing == Facing::South
            && self.r#open == false
            && self.r#hinge == Hinge::Left
            && self.r#powered == false
        {
            return 25848;
        }
        if self.r#facing == Facing::West
            && self.r#hinge == Hinge::Left
            && self.r#powered == false
            && self.r#half == Half::Upper
            && self.r#open == false
        {
            return 25856;
        }
        if self.r#half == Half::Upper
            && self.r#open == false
            && self.r#facing == Facing::West
            && self.r#hinge == Hinge::Right
            && self.r#powered == false
        {
            return 25860;
        }
        if self.r#hinge == Hinge::Left
            && self.r#open == true
            && self.r#half == Half::Upper
            && self.r#facing == Facing::West
            && self.r#powered == false
        {
            return 25854;
        }
        if self.r#hinge == Hinge::Right
            && self.r#facing == Facing::North
            && self.r#open == false
            && self.r#half == Half::Lower
            && self.r#powered == true
        {
            return 25835;
        }
        if self.r#open == true
            && self.r#half == Half::Lower
            && self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#hinge == Hinge::Right
        {
            return 25834;
        }
        if self.r#open == true
            && self.r#powered == true
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::East
        {
            return 25877;
        }
        if self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
            && self.r#open == false
            && self.r#powered == true
            && self.r#facing == Facing::East
        {
            return 25883;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Left
            && self.r#powered == false
            && self.r#open == false
        {
            return 25824;
        }
        if self.r#half == Half::Upper
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#open == true
        {
            return 25869;
        }
        if self.r#powered == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
            && self.r#open == false
        {
            return 25852;
        }
        if self.r#half == Half::Lower
            && self.r#hinge == Hinge::Left
            && self.r#open == false
            && self.r#powered == true
            && self.r#facing == Facing::North
        {
            return 25831;
        }
        if self.r#half == Half::Lower
            && self.r#facing == Facing::West
            && self.r#open == true
            && self.r#powered == true
            && self.r#hinge == Hinge::Left
        {
            return 25861;
        }
        if self.r#powered == false
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Lower
            && self.r#facing == Facing::West
            && self.r#open == true
        {
            return 25866;
        }
        if self.r#half == Half::Lower
            && self.r#open == false
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::South
            && self.r#powered == true
        {
            return 25851;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Lower
            && self.r#open == true
            && self.r#hinge == Hinge::Left
            && self.r#powered == false
        {
            return 25846;
        }
        if self.r#powered == true
            && self.r#hinge == Hinge::Left
            && self.r#open == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Lower
        {
            return 25863;
        }
        if self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#open == true
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
        {
            return 25874;
        }
        if self.r#open == true
            && self.r#half == Half::Upper
            && self.r#powered == true
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::South
        {
            return 25837;
        }
        if self.r#powered == true
            && self.r#open == false
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::South
            && self.r#half == Half::Upper
        {
            return 25843;
        }
        if self.r#facing == Facing::North
            && self.r#open == true
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Lower
            && self.r#powered == true
        {
            return 25829;
        }
        if self.r#hinge == Hinge::Left
            && self.r#facing == Facing::West
            && self.r#half == Half::Upper
            && self.r#open == true
            && self.r#powered == true
        {
            return 25853;
        }
        if self.r#facing == Facing::East
            && self.r#open == true
            && self.r#powered == false
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Lower
        {
            return 25882;
        }
        if self.r#facing == Facing::East
            && self.r#open == false
            && self.r#powered == true
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
        {
            return 25875;
        }
        if self.r#open == false
            && self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Upper
        {
            return 25876;
        }
        if self.r#hinge == Hinge::Right
            && self.r#facing == Facing::East
            && self.r#open == true
            && self.r#half == Half::Upper
            && self.r#powered == true
        {
            return 25873;
        }
        if self.r#open == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Left
            && self.r#powered == true
        {
            return 25855;
        }
        if self.r#powered == false
            && self.r#half == Half::Lower
            && self.r#facing == Facing::East
            && self.r#open == true
            && self.r#hinge == Hinge::Left
        {
            return 25878;
        }
        if self.r#hinge == Hinge::Left
            && self.r#facing == Facing::North
            && self.r#half == Half::Upper
            && self.r#open == false
            && self.r#powered == true
        {
            return 25823;
        }
        if self.r#facing == Facing::West
            && self.r#open == true
            && self.r#powered == true
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Lower
        {
            return 25865;
        }
        if self.r#half == Half::Lower
            && self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#hinge == Hinge::Right
            && self.r#open == false
        {
            return 25836;
        }
        if self.r#open == false
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#half == Half::Upper
        {
            return 25827;
        }
        if self.r#open == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
            && self.r#powered == false
        {
            return 25858;
        }
        if self.r#powered == false
            && self.r#half == Half::Upper
            && self.r#open == true
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::East
        {
            return 25870;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Upper
            && self.r#powered == false
            && self.r#hinge == Hinge::Right
            && self.r#open == false
        {
            return 25828;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Upper
            && self.r#open == false
            && self.r#powered == false
            && self.r#hinge == Hinge::Left
        {
            return 25840;
        }
        if self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
            && self.r#open == true
            && self.r#facing == Facing::South
            && self.r#powered == false
        {
            return 25842;
        }
        if self.r#half == Half::Lower
            && self.r#facing == Facing::West
            && self.r#hinge == Hinge::Left
            && self.r#open == true
            && self.r#powered == false
        {
            return 25862;
        }
        if self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
            && self.r#powered == false
            && self.r#open == false
            && self.r#facing == Facing::West
        {
            return 25868;
        }
        if self.r#open == false
            && self.r#facing == Facing::South
            && self.r#hinge == Hinge::Right
            && self.r#powered == false
            && self.r#half == Half::Upper
        {
            return 25844;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25838 {
            return Some(CopperDoor {
                r#hinge: Hinge::Left,
                r#powered: false,
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 25859 {
            return Some(CopperDoor {
                r#open: false,
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25879 {
            return Some(CopperDoor {
                r#half: Half::Lower,
                r#open: false,
                r#facing: Facing::East,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25833 {
            return Some(CopperDoor {
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25826 {
            return Some(CopperDoor {
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#facing: Facing::North,
            });
        }
        if state_id == 25857 {
            return Some(CopperDoor {
                r#open: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 25847 {
            return Some(CopperDoor {
                r#open: false,
                r#facing: Facing::South,
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25871 {
            return Some(CopperDoor {
                r#open: false,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 25822 {
            return Some(CopperDoor {
                r#powered: false,
                r#facing: Facing::North,
                r#open: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25872 {
            return Some(CopperDoor {
                r#facing: Facing::East,
                r#open: false,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 25880 {
            return Some(CopperDoor {
                r#open: false,
                r#facing: Facing::East,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
            });
        }
        if state_id == 25864 {
            return Some(CopperDoor {
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
            });
        }
        if state_id == 25881 {
            return Some(CopperDoor {
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25850 {
            return Some(CopperDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 25845 {
            return Some(CopperDoor {
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::South,
            });
        }
        if state_id == 25841 {
            return Some(CopperDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#powered: true,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 25867 {
            return Some(CopperDoor {
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Lower,
                r#open: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25839 {
            return Some(CopperDoor {
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Upper,
            });
        }
        if state_id == 25832 {
            return Some(CopperDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: false,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 25849 {
            return Some(CopperDoor {
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#powered: true,
            });
        }
        if state_id == 25830 {
            return Some(CopperDoor {
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25821 {
            return Some(CopperDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 25884 {
            return Some(CopperDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 25825 {
            return Some(CopperDoor {
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#half: Half::Upper,
            });
        }
        if state_id == 25848 {
            return Some(CopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#open: false,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 25856 {
            return Some(CopperDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#half: Half::Upper,
                r#open: false,
            });
        }
        if state_id == 25860 {
            return Some(CopperDoor {
                r#half: Half::Upper,
                r#open: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#powered: false,
            });
        }
        if state_id == 25854 {
            return Some(CopperDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 25835 {
            return Some(CopperDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#open: false,
                r#half: Half::Lower,
                r#powered: true,
            });
        }
        if state_id == 25834 {
            return Some(CopperDoor {
                r#open: true,
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25877 {
            return Some(CopperDoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 25883 {
            return Some(CopperDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 25824 {
            return Some(CopperDoor {
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 25869 {
            return Some(CopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 25852 {
            return Some(CopperDoor {
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 25831 {
            return Some(CopperDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 25861 {
            return Some(CopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25866 {
            return Some(CopperDoor {
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 25851 {
            return Some(CopperDoor {
                r#half: Half::Lower,
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 25846 {
            return Some(CopperDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 25863 {
            return Some(CopperDoor {
                r#powered: true,
                r#hinge: Hinge::Left,
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Lower,
            });
        }
        if state_id == 25874 {
            return Some(CopperDoor {
                r#facing: Facing::East,
                r#powered: false,
                r#open: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25837 {
            return Some(CopperDoor {
                r#open: true,
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
            });
        }
        if state_id == 25843 {
            return Some(CopperDoor {
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#half: Half::Upper,
            });
        }
        if state_id == 25829 {
            return Some(CopperDoor {
                r#facing: Facing::North,
                r#open: true,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#powered: true,
            });
        }
        if state_id == 25853 {
            return Some(CopperDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 25882 {
            return Some(CopperDoor {
                r#facing: Facing::East,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
            });
        }
        if state_id == 25875 {
            return Some(CopperDoor {
                r#facing: Facing::East,
                r#open: false,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 25876 {
            return Some(CopperDoor {
                r#open: false,
                r#facing: Facing::East,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
            });
        }
        if state_id == 25873 {
            return Some(CopperDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 25855 {
            return Some(CopperDoor {
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 25878 {
            return Some(CopperDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#open: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25823 {
            return Some(CopperDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 25865 {
            return Some(CopperDoor {
                r#facing: Facing::West,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
            });
        }
        if state_id == 25836 {
            return Some(CopperDoor {
                r#half: Half::Lower,
                r#powered: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 25827 {
            return Some(CopperDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 25858 {
            return Some(CopperDoor {
                r#open: true,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#powered: false,
            });
        }
        if state_id == 25870 {
            return Some(CopperDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#open: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
            });
        }
        if state_id == 25828 {
            return Some(CopperDoor {
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 25840 {
            return Some(CopperDoor {
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 25842 {
            return Some(CopperDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 25862 {
            return Some(CopperDoor {
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 25868 {
            return Some(CopperDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 25844 {
            return Some(CopperDoor {
                r#open: false,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        return None;
    }
}
