use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredCopperTrapdoor {
    pub waterlogged: bool,
    pub r#half: Half,
    pub powered: bool,
    pub open: bool,
    pub r#facing: Facing,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for WaxedWeatheredCopperTrapdoor {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#open == true && self.r#half == Half::Top && self.r#powered == true && self.r#facing == Facing::South { return 26798; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#open == true && self.r#powered == true { return 26789; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#open == true && self.r#powered == true && self.r#waterlogged == true { return 26781; }
        if self.r#open == false && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#powered == false && self.r#waterlogged == false { return 26804; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#open == true && self.r#facing == Facing::South && self.r#powered == true { return 26797; }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#open == false && self.r#waterlogged == false { return 26802; }
        if self.r#half == Half::Bottom && self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#open == false { return 26796; }
        if self.r#open == true && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#powered == false { return 26792; }
        if self.r#waterlogged == false && self.r#open == false && self.r#powered == true && self.r#facing == Facing::South && self.r#half == Half::Bottom { return 26810; }
        if self.r#facing == Facing::West && self.r#open == true && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#powered == true { return 26821; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#powered == true && self.r#open == true && self.r#facing == Facing::West { return 26813; }
        if self.r#facing == Facing::East && self.r#powered == true && self.r#open == false && self.r#waterlogged == true && self.r#half == Half::Bottom { return 26841; }
        if self.r#half == Half::Bottom && self.r#powered == false && self.r#waterlogged == true && self.r#open == false && self.r#facing == Facing::East { return 26843; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#powered == true && self.r#open == false { return 26786; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#open == false && self.r#powered == true && self.r#waterlogged == true { return 26785; }
        if self.r#waterlogged == false && self.r#powered == true && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#open == false { return 26818; }
        if self.r#open == true && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#powered == true && self.r#waterlogged == false { return 26806; }
        if self.r#powered == false && self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#open == true { return 26816; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#powered == false && self.r#open == false && self.r#half == Half::Top { return 26819; }
        if self.r#open == false && self.r#powered == false && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Bottom { return 26811; }
        if self.r#half == Half::Top && self.r#open == true && self.r#facing == Facing::North && self.r#powered == false && self.r#waterlogged == false { return 26784; }
        if self.r#powered == true && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#open == false && self.r#waterlogged == false { return 26842; }
        if self.r#half == Half::Top && self.r#powered == true && self.r#facing == Facing::South && self.r#open == false && self.r#waterlogged == true { return 26801; }
        if self.r#open == false && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#powered == false { return 26795; }
        if self.r#half == Half::Bottom && self.r#powered == false && self.r#open == true && self.r#waterlogged == true && self.r#facing == Facing::North { return 26791; }
        if self.r#powered == true && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#open == false { return 26793; }
        if self.r#half == Half::Top && self.r#open == false && self.r#powered == true && self.r#waterlogged == true && self.r#facing == Facing::West { return 26817; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#open == true && self.r#half == Half::Top && self.r#powered == true { return 26782; }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#half == Half::Bottom && self.r#open == true && self.r#waterlogged == true { return 26839; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#open == true && self.r#powered == false { return 26823; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#open == true && self.r#powered == false && self.r#half == Half::Top { return 26832; }
        if self.r#facing == Facing::North && self.r#open == true && self.r#powered == false && self.r#waterlogged == true && self.r#half == Half::Top { return 26783; }
        if self.r#powered == false && self.r#waterlogged == true && self.r#open == false && self.r#half == Half::Bottom && self.r#facing == Facing::West { return 26827; }
        if self.r#half == Half::Bottom && self.r#open == true && self.r#powered == true && self.r#waterlogged == true && self.r#facing == Facing::East { return 26837; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#powered == true && self.r#open == true { return 26814; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#powered == true && self.r#half == Half::Bottom && self.r#open == true { return 26838; }
        if self.r#half == Half::Bottom && self.r#powered == true && self.r#open == true && self.r#waterlogged == false && self.r#facing == Facing::West { return 26822; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#open == true && self.r#half == Half::Top { return 26800; }
        if self.r#open == false && self.r#powered == false && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == false { return 26788; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#powered == false && self.r#open == false && self.r#half == Half::Top { return 26835; }
        if self.r#facing == Facing::South && self.r#powered == false && self.r#waterlogged == false && self.r#open == true && self.r#half == Half::Bottom { return 26808; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#half == Half::Top && self.r#powered == true && self.r#open == false { return 26833; }
        if self.r#half == Half::Top && self.r#powered == false && self.r#open == true && self.r#waterlogged == true && self.r#facing == Facing::South { return 26799; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#open == false && self.r#facing == Facing::South && self.r#powered == false { return 26812; }
        if self.r#open == true && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#powered == false { return 26824; }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#open == false && self.r#half == Half::Bottom && self.r#waterlogged == false { return 26794; }
        if self.r#facing == Facing::East && self.r#powered == true && self.r#waterlogged == false && self.r#half == Half::Top && self.r#open == false { return 26834; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#powered == false && self.r#half == Half::Bottom && self.r#open == true { return 26840; }
        if self.r#open == false && self.r#powered == false && self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::West { return 26820; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#open == false && self.r#powered == true && self.r#facing == Facing::West { return 26825; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#powered == true && self.r#facing == Facing::East && self.r#open == true { return 26830; }
        if self.r#open == true && self.r#facing == Facing::South && self.r#powered == false && self.r#waterlogged == true && self.r#half == Half::Bottom { return 26807; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#open == false && self.r#powered == false { return 26803; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#open == true && self.r#powered == false { return 26831; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == true && self.r#powered == false && self.r#open == true { return 26815; }
        if self.r#powered == true && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#open == true && self.r#half == Half::Bottom { return 26805; }
        if self.r#waterlogged == false && self.r#open == false && self.r#powered == false && self.r#facing == Facing::East && self.r#half == Half::Bottom { return 26844; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#powered == false && self.r#open == false { return 26836; }
        if self.r#waterlogged == false && self.r#powered == true && self.r#open == false && self.r#half == Half::Bottom && self.r#facing == Facing::West { return 26826; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#open == false && self.r#powered == false && self.r#waterlogged == true { return 26787; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#powered == false && self.r#open == false && self.r#facing == Facing::West { return 26828; }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#open == false { return 26809; }
        if self.r#half == Half::Top && self.r#open == true && self.r#facing == Facing::East && self.r#powered == true && self.r#waterlogged == true { return 26829; }
        if self.r#powered == true && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#open == true { return 26790; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26798 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 26789 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26781 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: true,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26804 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26797 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 26802 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26796 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 26792 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#open: true,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 26810 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26821 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::West,
                r#open: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 26813 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#powered: true,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 26841 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::East,
                r#powered: true,
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26843 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 26786 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 26785 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26818 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 26806 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#open: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26816 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 26819 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: false,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26811 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#open: false,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26784 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26842 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26801 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::South,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26795 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 26791 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26793 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 26817 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 26782 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#open: true,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 26839 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26823 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 26832 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#open: true,
                r#powered: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26783 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::North,
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26827 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 26837 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 26814 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 26838 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: true,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 26822 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 26800 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26788 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#open: false,
                r#powered: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 26835 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: false,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26808 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26833 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 26799 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 26812 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 26824 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 26794 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::North,
                r#powered: true,
                r#open: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 26834 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 26840 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: false,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 26820 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 26825 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: false,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 26830 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 26807 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#open: true,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26803 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26831 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 26815 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 26805 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26844 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26836 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 26826 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: false,
                r#powered: true,
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 26787 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: false,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26828 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#powered: false,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 26809 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 26829 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26790 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        return None;
    }
}

