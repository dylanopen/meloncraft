use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedCopperTrapdoor {
    pub waterlogged: bool,
    pub open: bool,
    pub r#facing: Facing,
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

impl BlockState for WaxedCopperTrapdoor {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#open == false { return 26594; }
        if block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 26598; }
        if block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom { return 26633; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 26589; }
        if block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 26604; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == false { return 26596; }
        if block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 26609; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#open == false { return 26612; }
        if block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#open == false { return 26641; }
        if block_state.r#powered == true && block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 26630; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom { return 26599; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == true { return 26615; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#powered == false { return 26603; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#open == false { return 26636; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 26649; }
        if block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 26651; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 26627; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#facing == Facing::North { return 26592; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#open == false { return 26620; }
        if block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#open == true { return 26623; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 26617; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#open == false { return 26626; }
        if block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 26643; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == true { return 26595; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#open == false && block_state.r#waterlogged == true { return 26601; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Top { return 26606; }
        if block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#powered == true { return 26610; }
        if block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 26624; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == true { return 26634; }
        if block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#waterlogged == false { return 26644; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#open == true { return 26632; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::South { return 26608; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#open == true { return 26646; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#open == true { return 26605; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::South { return 26618; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#open == true { return 26607; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#open == true { return 26629; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#open == false { return 26619; }
        if block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#waterlogged == true { return 26613; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#open == false { return 26593; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#open == true { return 26600; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#open == true { return 26637; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == true { return 26614; }
        if block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#powered == true { return 26642; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#open == false { return 26652; }
        if block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#facing == Facing::West { return 26635; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == true { return 26597; }
        if block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#open == true { return 26640; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::North { return 26591; }
        if block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 26628; }
        if block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#open == true { return 26621; }
        if block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom { return 26645; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#open == true { return 26648; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#open == true { return 26622; }
        if block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#open == false { return 26650; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#open == false { return 26625; }
        if block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#open == true { return 26639; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#powered == true && block_state.r#waterlogged == false { return 26638; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#powered == false { return 26616; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#open == false { return 26602; }
        if block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Top { return 26590; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#waterlogged == true { return 26631; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#powered == false { return 26611; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 26647; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26594 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 26598 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 26633 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26589 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::North,
                r#powered: true,
                r#open: true,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 26604 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::North,
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26596 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26609 {
            return Some(WaxedCopperTrapdoor {
                r#open: false,
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 26612 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 26641 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 26630 {
            return Some(WaxedCopperTrapdoor {
                r#powered: true,
                r#open: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 26599 {
            return Some(WaxedCopperTrapdoor {
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26615 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26603 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#open: false,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 26636 {
            return Some(WaxedCopperTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 26649 {
            return Some(WaxedCopperTrapdoor {
                r#open: false,
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 26651 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 26627 {
            return Some(WaxedCopperTrapdoor {
                r#powered: false,
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 26592 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: false,
                r#powered: false,
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26620 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 26623 {
            return Some(WaxedCopperTrapdoor {
                r#powered: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 26617 {
            return Some(WaxedCopperTrapdoor {
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26626 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 26643 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#powered: false,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 26595 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: false,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26601 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: true,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26606 {
            return Some(WaxedCopperTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 26610 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 26624 {
            return Some(WaxedCopperTrapdoor {
                r#powered: false,
                r#half: Half::Top,
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 26634 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 26644 {
            return Some(WaxedCopperTrapdoor {
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26632 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 26608 {
            return Some(WaxedCopperTrapdoor {
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 26646 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 26605 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 26618 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 26607 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 26629 {
            return Some(WaxedCopperTrapdoor {
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 26619 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 26613 {
            return Some(WaxedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26593 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 26600 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 26637 {
            return Some(WaxedCopperTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 26614 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26642 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 26652 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 26635 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Bottom,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 26597 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 26640 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: false,
                r#powered: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 26591 {
            return Some(WaxedCopperTrapdoor {
                r#powered: false,
                r#open: true,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 26628 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 26621 {
            return Some(WaxedCopperTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 26645 {
            return Some(WaxedCopperTrapdoor {
                r#open: true,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26648 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 26622 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 26650 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: false,
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 26625 {
            return Some(WaxedCopperTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 26639 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 26638 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#open: true,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26616 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 26602 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 26590 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: false,
                r#open: true,
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 26631 {
            return Some(WaxedCopperTrapdoor {
                r#powered: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26611 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#open: false,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 26647 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::East,
                r#powered: false,
                r#open: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        return None;
    }
}

