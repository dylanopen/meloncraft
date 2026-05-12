use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredCopperTrapdoor {
    pub waterlogged: bool,
    pub powered: bool,
    pub r#half: Half,
    pub r#facing: Facing,
    pub open: bool,
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
    fn to_id(self) -> i32 {
        if block_state.r#powered == true && block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West { return 26822; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#open == false { return 26809; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 26787; }
        if block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#powered == true { return 26813; }
        if block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East { return 26838; }
        if block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#open == false { return 26802; }
        if block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#powered == false { return 26815; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#open == true && block_state.r#half == Half::Bottom { return 26790; }
        if block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 26801; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 26793; }
        if block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#open == false { return 26833; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#open == true { return 26800; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 26831; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#half == Half::Bottom { return 26794; }
        if block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#powered == true { return 26805; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#waterlogged == true { return 26797; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#open == true && block_state.r#waterlogged == true { return 26789; }
        if block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#open == false { return 26820; }
        if block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom { return 26827; }
        if block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 26795; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#powered == false { return 26839; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#open == false { return 26804; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom { return 26828; }
        if block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#open == true { return 26832; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 26835; }
        if block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::North { return 26781; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::South { return 26803; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#powered == true && block_state.r#waterlogged == false { return 26830; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#open == true { return 26783; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Top { return 26786; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#open == false { return 26841; }
        if block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#waterlogged == false { return 26782; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#waterlogged == true { return 26837; }
        if block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 26823; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#open == true { return 26799; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 26817; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#open == false { return 26796; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#open == false { return 26810; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East { return 26844; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#open == true && block_state.r#waterlogged == true { return 26807; }
        if block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Top { return 26816; }
        if block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#powered == true { return 26814; }
        if block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#powered == true { return 26842; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#open == false { return 26812; }
        if block_state.r#powered == true && block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West { return 26826; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#powered == true { return 26798; }
        if block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 26792; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#open == false { return 26819; }
        if block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 26808; }
        if block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#powered == false { return 26788; }
        if block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#half == Half::Top { return 26818; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Bottom { return 26840; }
        if block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#powered == false { return 26811; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#open == true && block_state.r#half == Half::Top { return 26784; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#open == false { return 26836; }
        if block_state.r#powered == true && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom { return 26821; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom { return 26843; }
        if block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#powered == true { return 26834; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#open == true { return 26824; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#half == Half::Top { return 26829; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::West { return 26825; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#open == true { return 26791; }
        if block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 26806; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::North { return 26785; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26822 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: true,
                r#open: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 26809 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: true,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 26787 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#open: false,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26813 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 26838 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: false,
                r#powered: true,
                r#open: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 26802 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 26815 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 26790 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#powered: true,
                r#open: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26801 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 26793 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#open: false,
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 26833 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 26800 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 26831 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: false,
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 26794 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: false,
                r#open: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26805 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 26797 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26789 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#powered: true,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26820 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 26827 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#powered: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26795 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::North,
                r#open: false,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26839 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 26804 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 26828 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26832 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 26835 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: false,
                r#open: false,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 26781 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#open: true,
                r#half: Half::Top,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26803 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#open: false,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 26830 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#open: true,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26783 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 26786 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#open: false,
                r#powered: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26841 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 26782 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26837 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26823 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#open: true,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 26799 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 26817 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: true,
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 26796 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 26810 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 26844 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 26807 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#powered: false,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26816 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 26814 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#open: true,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 26842 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 26812 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#powered: false,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 26826 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: true,
                r#open: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 26798 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 26792 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#open: true,
                r#half: Half::Bottom,
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 26819 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 26808 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 26788 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::North,
                r#open: false,
                r#half: Half::Top,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 26818 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26840 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#open: true,
                r#powered: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26811 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 26784 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: false,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26836 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 26821 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: true,
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26843 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: false,
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26834 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#waterlogged: false,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 26824 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 26829 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Top,
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
        if state_id == 26791 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 26806 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#open: true,
                r#half: Half::Bottom,
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 26785 {
            return Some(WaxedWeatheredCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

