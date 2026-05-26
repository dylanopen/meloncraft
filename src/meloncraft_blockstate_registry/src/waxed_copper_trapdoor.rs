use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedCopperTrapdoor {
    pub waterlogged: bool,
    pub r#half: Half,
    pub r#facing: Facing,
    pub open: bool,
    pub powered: bool,
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

impl BlockState for WaxedCopperTrapdoor {
    fn to_id(&self) -> i32 {
        if self.r#open == false
            && self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 26617;
        }
        if self.r#waterlogged == true
            && self.r#open == false
            && self.r#powered == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
        {
            return 26627;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#open == false
        {
            return 26625;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#powered == true
            && self.r#open == false
        {
            return 26593;
        }
        if self.r#half == Half::Top
            && self.r#open == true
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
        {
            return 26591;
        }
        if self.r#powered == true
            && self.r#open == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
        {
            return 26614;
        }
        if self.r#open == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 26622;
        }
        if self.r#open == true
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
        {
            return 26623;
        }
        if self.r#half == Half::Top
            && self.r#open == false
            && self.r#powered == true
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 26642;
        }
        if self.r#powered == true
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#open == true
        {
            return 26606;
        }
        if self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#open == true
        {
            return 26648;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#open == false
            && self.r#facing == Facing::East
            && self.r#powered == true
        {
            return 26650;
        }
        if self.r#open == false
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#powered == true
        {
            return 26633;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#open == true
        {
            return 26599;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#waterlogged == true
            && self.r#powered == true
        {
            return 26629;
        }
        if self.r#powered == false
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#open == true
        {
            return 26615;
        }
        if self.r#half == Half::Top
            && self.r#open == true
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
        {
            return 26639;
        }
        if self.r#powered == true
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#open == false
        {
            return 26626;
        }
        if self.r#open == true
            && self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 26605;
        }
        if self.r#facing == Facing::North
            && self.r#open == false
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 26601;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#open == false
        {
            return 26609;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#open == true
            && self.r#powered == false
        {
            return 26632;
        }
        if self.r#half == Half::Top
            && self.r#open == false
            && self.r#waterlogged == true
            && self.r#powered == false
            && self.r#facing == Facing::North
        {
            return 26595;
        }
        if self.r#powered == true
            && self.r#facing == Facing::North
            && self.r#open == true
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 26597;
        }
        if self.r#half == Half::Top
            && self.r#powered == false
            && self.r#open == true
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 26640;
        }
        if self.r#powered == false
            && self.r#open == false
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 26643;
        }
        if self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
        {
            return 26647;
        }
        if self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#half == Half::Bottom
            && self.r#open == false
            && self.r#waterlogged == false
        {
            return 26652;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#powered == true
            && self.r#half == Half::Top
            && self.r#open == true
        {
            return 26621;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#open == false
            && self.r#powered == false
        {
            return 26619;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#powered == true
            && self.r#open == false
            && self.r#half == Half::Top
        {
            return 26610;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#open == true
            && self.r#powered == false
        {
            return 26631;
        }
        if self.r#open == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#half == Half::Bottom
        {
            return 26630;
        }
        if self.r#open == false
            && self.r#half == Half::Bottom
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
        {
            return 26603;
        }
        if self.r#facing == Facing::West
            && self.r#open == false
            && self.r#powered == false
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 26635;
        }
        if self.r#facing == Facing::South
            && self.r#open == false
            && self.r#waterlogged == false
            && self.r#powered == true
            && self.r#half == Half::Bottom
        {
            return 26618;
        }
        if self.r#open == false
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Top
        {
            return 26641;
        }
        if self.r#open == true
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::North
        {
            return 26589;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#open == true
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 26608;
        }
        if self.r#open == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#waterlogged == false
        {
            return 26638;
        }
        if self.r#half == Half::Top
            && self.r#powered == false
            && self.r#open == false
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 26611;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#powered == true
            && self.r#open == true
        {
            return 26637;
        }
        if self.r#open == false
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#powered == true
        {
            return 26602;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#open == true
        {
            return 26613;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#open == false
            && self.r#waterlogged == true
            && self.r#powered == false
        {
            return 26651;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#open == true
            && self.r#powered == false
            && self.r#waterlogged == true
        {
            return 26607;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#open == true
            && self.r#powered == false
        {
            return 26616;
        }
        if self.r#open == false
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#powered == false
        {
            return 26604;
        }
        if self.r#open == false
            && self.r#powered == false
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::North
        {
            return 26596;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#waterlogged == true
            && self.r#open == false
        {
            return 26649;
        }
        if self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#open == true
        {
            return 26646;
        }
        if self.r#powered == true
            && self.r#half == Half::Top
            && self.r#open == false
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 26594;
        }
        if self.r#half == Half::Top
            && self.r#open == false
            && self.r#powered == false
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
        {
            return 26612;
        }
        if self.r#half == Half::Top
            && self.r#powered == false
            && self.r#open == true
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 26624;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#open == true
            && self.r#half == Half::Top
            && self.r#powered == true
        {
            return 26590;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#powered == false
            && self.r#open == false
            && self.r#half == Half::Top
        {
            return 26644;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#open == false
            && self.r#powered == false
            && self.r#waterlogged == false
        {
            return 26628;
        }
        if self.r#half == Half::Bottom
            && self.r#powered == false
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#open == true
        {
            return 26600;
        }
        if self.r#powered == true
            && self.r#half == Half::Bottom
            && self.r#open == false
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
        {
            return 26634;
        }
        if self.r#waterlogged == false
            && self.r#open == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#powered == false
        {
            return 26592;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::West
            && self.r#open == false
            && self.r#powered == false
        {
            return 26636;
        }
        if self.r#open == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#powered == true
            && self.r#waterlogged == true
        {
            return 26645;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#powered == false
            && self.r#open == false
            && self.r#waterlogged == false
        {
            return 26620;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#open == true
            && self.r#powered == true
            && self.r#half == Half::Bottom
        {
            return 26598;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26617 {
            return Some(WaxedCopperTrapdoor {
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 26627 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#powered: false,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 26625 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 26593 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 26591 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26614 {
            return Some(WaxedCopperTrapdoor {
                r#powered: true,
                r#open: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 26622 {
            return Some(WaxedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26623 {
            return Some(WaxedCopperTrapdoor {
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 26642 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 26606 {
            return Some(WaxedCopperTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 26648 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 26650 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 26633 {
            return Some(WaxedCopperTrapdoor {
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#powered: true,
            });
        }
        if state_id == 26599 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 26629 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 26615 {
            return Some(WaxedCopperTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 26639 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 26626 {
            return Some(WaxedCopperTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 26605 {
            return Some(WaxedCopperTrapdoor {
                r#open: true,
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 26601 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26609 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 26632 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 26595 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 26597 {
            return Some(WaxedCopperTrapdoor {
                r#powered: true,
                r#facing: Facing::North,
                r#open: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 26640 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#open: true,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 26643 {
            return Some(WaxedCopperTrapdoor {
                r#powered: false,
                r#open: false,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 26647 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 26652 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26621 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: true,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 26619 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26610 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#powered: true,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26631 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 26630 {
            return Some(WaxedCopperTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26603 {
            return Some(WaxedCopperTrapdoor {
                r#open: false,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 26635 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26618 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::South,
                r#open: false,
                r#waterlogged: false,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 26641 {
            return Some(WaxedCopperTrapdoor {
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 26589 {
            return Some(WaxedCopperTrapdoor {
                r#open: true,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 26608 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26638 {
            return Some(WaxedCopperTrapdoor {
                r#open: true,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 26611 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 26637 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 26602 {
            return Some(WaxedCopperTrapdoor {
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: true,
            });
        }
        if state_id == 26613 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 26651 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#open: false,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 26607 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 26616 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 26604 {
            return Some(WaxedCopperTrapdoor {
                r#open: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 26596 {
            return Some(WaxedCopperTrapdoor {
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 26649 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 26646 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 26594 {
            return Some(WaxedCopperTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#open: false,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 26612 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 26624 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#open: true,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 26590 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#open: true,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 26644 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: false,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 26628 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26600 {
            return Some(WaxedCopperTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 26634 {
            return Some(WaxedCopperTrapdoor {
                r#powered: true,
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 26592 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 26636 {
            return Some(WaxedCopperTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 26645 {
            return Some(WaxedCopperTrapdoor {
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 26620 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#powered: false,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 26598 {
            return Some(WaxedCopperTrapdoor {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#open: true,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        return None;
    }
}
