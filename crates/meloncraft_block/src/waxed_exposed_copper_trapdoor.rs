use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedExposedCopperTrapdoor {
    pub powered: bool,
    pub r#facing: Facing,
    pub waterlogged: bool,
    pub r#half: Half,
    pub open: bool,
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
    Top,
    Bottom,
}

impl BlockState for WaxedExposedCopperTrapdoor {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 26676; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 26685; }
        if block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#powered == true { return 26665; }
        if block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#half == Half::Top { return 26687; }
        if block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#powered == false { return 26675; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Top { return 26704; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 26695; }
        if block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#powered == false { return 26703; }
        if block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#powered == true { return 26701; }
        if block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#powered == false { return 26664; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#powered == true { return 26669; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == true { return 26689; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 26682; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#waterlogged == false { return 26710; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#open == true { return 26653; }
        if block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#powered == false { return 26692; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#open == true { return 26679; }
        if block_state.r#powered == true && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Top { return 26705; }
        if block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 26663; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#facing == Facing::South { return 26671; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == true { return 26698; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#powered == true { return 26693; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#open == true { return 26680; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#open == false { return 26666; }
        if block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#waterlogged == false { return 26678; }
        if block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 26700; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East { return 26712; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#open == false { return 26684; }
        if block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#waterlogged == false { return 26672; }
        if block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#powered == false { return 26699; }
        if block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#half == Half::Top { return 26670; }
        if block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#open == false { return 26690; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#powered == true { return 26686; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == true { return 26697; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 26667; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#waterlogged == true { return 26709; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#open == false { return 26660; }
        if block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 26658; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#open == true { return 26654; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#open == true { return 26655; }
        if block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 26694; }
        if block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 26677; }
        if block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 26696; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#half == Half::Top { return 26659; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#open == false { return 26673; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#open == false { return 26707; }
        if block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#half == Half::Bottom { return 26713; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East { return 26715; }
        if block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#waterlogged == false { return 26708; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#open == false { return 26716; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 26711; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North { return 26661; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 26691; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#powered == true { return 26681; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 26683; }
        if block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::North { return 26657; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#waterlogged == false { return 26668; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 26656; }
        if block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 26706; }
        if block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#facing == Facing::East { return 26714; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::North { return 26662; }
        if block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#open == false { return 26674; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 26702; }
        if block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 26688; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26676 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::South,
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26685 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::West,
                r#powered: true,
                r#open: true,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 26665 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 26687 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::West,
                r#powered: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26675 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 26704 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#open: true,
                r#powered: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26695 {
            return Some(WaxedExposedCopperTrapdoor {
                r#powered: false,
                r#open: true,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26703 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 26701 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 26664 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 26669 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26689 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26682 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26710 {
            return Some(WaxedExposedCopperTrapdoor {
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26653 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 26692 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 26679 {
            return Some(WaxedExposedCopperTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 26705 {
            return Some(WaxedExposedCopperTrapdoor {
                r#powered: true,
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 26663 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 26671 {
            return Some(WaxedExposedCopperTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 26698 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26693 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 26680 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 26666 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 26678 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26700 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26712 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 26684 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#powered: false,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 26672 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26699 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 26670 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::South,
                r#open: true,
                r#waterlogged: false,
                r#powered: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26690 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 26686 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26697 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26667 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::North,
                r#powered: false,
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26709 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26660 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 26658 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: false,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 26654 {
            return Some(WaxedExposedCopperTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 26655 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 26694 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26677 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 26696 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 26659 {
            return Some(WaxedExposedCopperTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26673 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::South,
                r#powered: true,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 26707 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 26713 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#waterlogged: true,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26715 {
            return Some(WaxedExposedCopperTrapdoor {
                r#powered: false,
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 26708 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26716 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 26711 {
            return Some(WaxedExposedCopperTrapdoor {
                r#powered: false,
                r#open: true,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26661 {
            return Some(WaxedExposedCopperTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#open: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 26691 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::West,
                r#powered: false,
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26681 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Bottom,
                r#powered: true,
            });
        }
        if state_id == 26683 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: false,
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 26657 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#powered: true,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 26668 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26656 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: true,
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 26706 {
            return Some(WaxedExposedCopperTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#open: false,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 26714 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#half: Half::Bottom,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 26662 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26674 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 26702 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 26688 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

