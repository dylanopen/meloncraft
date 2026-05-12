use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedExposedCopperTrapdoor {
    pub r#facing: Facing,
    pub open: bool,
    pub waterlogged: bool,
    pub powered: bool,
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
pub enum Half {
    Top,
    Bottom,
}

impl BlockState for WaxedExposedCopperTrapdoor {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#open == true && self.r#half == Half::Bottom && self.r#powered == false { return 26663; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#open == true && self.r#powered == true && self.r#waterlogged == true { return 26661; }
        if self.r#waterlogged == true && self.r#open == false && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#powered == true { return 26673; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#open == true && self.r#powered == false { return 26688; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#powered == true && self.r#open == true { return 26678; }
        if self.r#open == true && self.r#powered == true && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Top { return 26653; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#open == false && self.r#powered == false { return 26676; }
        if self.r#facing == Facing::West && self.r#powered == false && self.r#half == Half::Bottom && self.r#open == false && self.r#waterlogged == true { return 26699; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#powered == false && self.r#open == false && self.r#waterlogged == false { return 26692; }
        if self.r#open == true && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#powered == true && self.r#half == Half::Bottom { return 26693; }
        if self.r#powered == true && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#waterlogged == false && self.r#open == false { return 26658; }
        if self.r#open == true && self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Bottom { return 26712; }
        if self.r#waterlogged == true && self.r#open == false && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#powered == false { return 26691; }
        if self.r#facing == Facing::South && self.r#powered == false && self.r#waterlogged == true && self.r#open == true && self.r#half == Half::Top { return 26671; }
        if self.r#half == Half::Bottom && self.r#open == false && self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::South { return 26684; }
        if self.r#waterlogged == false && self.r#open == true && self.r#facing == Facing::South && self.r#powered == false && self.r#half == Half::Top { return 26672; }
        if self.r#half == Half::Top && self.r#powered == true && self.r#open == false && self.r#waterlogged == false && self.r#facing == Facing::South { return 26674; }
        if self.r#powered == true && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#open == true { return 26710; }
        if self.r#powered == false && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#half == Half::Top && self.r#open == false { return 26675; }
        if self.r#open == false && self.r#powered == true && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Top { return 26705; }
        if self.r#powered == true && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#open == true && self.r#facing == Facing::South { return 26677; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#open == false && self.r#waterlogged == false && self.r#powered == false { return 26660; }
        if self.r#half == Half::Top && self.r#open == false && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#powered == false { return 26708; }
        if self.r#powered == true && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#open == true { return 26709; }
        if self.r#open == false && self.r#waterlogged == true && self.r#half == Half::Top && self.r#powered == true && self.r#facing == Facing::North { return 26657; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#open == false && self.r#powered == true && self.r#half == Half::Bottom { return 26698; }
        if self.r#open == true && self.r#facing == Facing::East && self.r#powered == false && self.r#waterlogged == true && self.r#half == Half::Bottom { return 26711; }
        if self.r#open == true && self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Top { return 26686; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#open == false && self.r#powered == false && self.r#waterlogged == false { return 26700; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#powered == false && self.r#open == true && self.r#waterlogged == false { return 26704; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#open == false && self.r#powered == true { return 26666; }
        if self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#open == true { return 26662; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#powered == false && self.r#open == true { return 26687; }
        if self.r#facing == Facing::North && self.r#open == true && self.r#half == Half::Bottom && self.r#powered == false && self.r#waterlogged == false { return 26664; }
        if self.r#half == Half::Top && self.r#powered == true && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#open == false { return 26689; }
        if self.r#open == true && self.r#half == Half::Top && self.r#powered == false && self.r#facing == Facing::North && self.r#waterlogged == true { return 26655; }
        if self.r#facing == Facing::East && self.r#open == false && self.r#waterlogged == true && self.r#half == Half::Top && self.r#powered == false { return 26707; }
        if self.r#waterlogged == false && self.r#open == false && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#powered == true { return 26706; }
        if self.r#open == true && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#powered == true && self.r#half == Half::Top { return 26654; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#powered == true && self.r#open == false { return 26697; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#open == false && self.r#powered == false && self.r#waterlogged == true { return 26659; }
        if self.r#powered == false && self.r#open == false && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#waterlogged == true { return 26667; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#open == true && self.r#waterlogged == true && self.r#powered == true { return 26685; }
        if self.r#half == Half::Bottom && self.r#powered == false && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#open == false { return 26683; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#powered == true && self.r#waterlogged == true && self.r#open == true { return 26701; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#open == false && self.r#powered == false { return 26668; }
        if self.r#waterlogged == false && self.r#powered == true && self.r#facing == Facing::East && self.r#open == true && self.r#half == Half::Top { return 26702; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#powered == false && self.r#open == true { return 26656; }
        if self.r#open == true && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#powered == false && self.r#waterlogged == false { return 26696; }
        if self.r#half == Half::Bottom && self.r#powered == true && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#open == false { return 26665; }
        if self.r#facing == Facing::West && self.r#open == false && self.r#powered == true && self.r#waterlogged == false && self.r#half == Half::Top { return 26690; }
        if self.r#powered == true && self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#open == true { return 26670; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == true && self.r#open == true && self.r#powered == false { return 26703; }
        if self.r#open == true && self.r#waterlogged == true && self.r#powered == false && self.r#half == Half::Bottom && self.r#facing == Facing::West { return 26695; }
        if self.r#facing == Facing::East && self.r#powered == true && self.r#open == false && self.r#waterlogged == true && self.r#half == Half::Bottom { return 26713; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#open == false && self.r#powered == true { return 26681; }
        if self.r#powered == true && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#open == false { return 26682; }
        if self.r#facing == Facing::East && self.r#powered == false && self.r#open == false && self.r#waterlogged == true && self.r#half == Half::Bottom { return 26715; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#open == true { return 26694; }
        if self.r#half == Half::Top && self.r#powered == true && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#open == true { return 26669; }
        if self.r#waterlogged == false && self.r#powered == false && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#open == false { return 26716; }
        if self.r#open == false && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#powered == true && self.r#waterlogged == false { return 26714; }
        if self.r#half == Half::Bottom && self.r#open == true && self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::South { return 26680; }
        if self.r#powered == false && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#open == true { return 26679; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26663 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#open: true,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 26661 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26673 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 26688 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 26678 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 26653 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: true,
                r#powered: true,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26676 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26699 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::West,
                r#powered: false,
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26692 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#powered: false,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26693 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26658 {
            return Some(WaxedExposedCopperTrapdoor {
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 26712 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26691 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 26671 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26684 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 26672 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26674 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 26710 {
            return Some(WaxedExposedCopperTrapdoor {
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 26675 {
            return Some(WaxedExposedCopperTrapdoor {
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 26705 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 26677 {
            return Some(WaxedExposedCopperTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 26660 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 26708 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 26709 {
            return Some(WaxedExposedCopperTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 26657 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26698 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26711 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26686 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: true,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 26700 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26704 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#powered: false,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26666 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26662 {
            return Some(WaxedExposedCopperTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 26687 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 26664 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::North,
                r#open: true,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26689 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 26655 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: true,
                r#half: Half::Top,
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 26707 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 26706 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 26654 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26697 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 26659 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#open: false,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26667 {
            return Some(WaxedExposedCopperTrapdoor {
                r#powered: false,
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 26685 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#open: true,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 26683 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 26701 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 26668 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26702 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26656 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 26696 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26665 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 26690 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26670 {
            return Some(WaxedExposedCopperTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 26703 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 26695 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: true,
                r#waterlogged: true,
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 26713 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::East,
                r#powered: true,
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26681 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26682 {
            return Some(WaxedExposedCopperTrapdoor {
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 26715 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::East,
                r#powered: false,
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26694 {
            return Some(WaxedExposedCopperTrapdoor {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 26669 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 26716 {
            return Some(WaxedExposedCopperTrapdoor {
                r#waterlogged: false,
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 26714 {
            return Some(WaxedExposedCopperTrapdoor {
                r#open: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26680 {
            return Some(WaxedExposedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 26679 {
            return Some(WaxedExposedCopperTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        return None;
    }
}

