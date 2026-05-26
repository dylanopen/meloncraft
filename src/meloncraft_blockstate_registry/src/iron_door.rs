use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IronDoor {
    pub open: bool,
    pub powered: bool,
    pub r#facing: Facing,
    pub r#hinge: Hinge,
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

impl BlockState for IronDoor {
    fn to_id(&self) -> i32 {
        if self.r#hinge == Hinge::Right
            && self.r#open == false
            && self.r#half == Half::Lower
            && self.r#facing == Facing::South
            && self.r#powered == true
        {
            return 6626;
        }
        if self.r#facing == Facing::East
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Lower
            && self.r#powered == true
            && self.r#open == false
        {
            return 6658;
        }
        if self.r#hinge == Hinge::Left
            && self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#open == true
            && self.r#half == Half::Lower
        {
            return 6636;
        }
        if self.r#half == Half::Upper
            && self.r#facing == Facing::West
            && self.r#open == false
            && self.r#powered == false
            && self.r#hinge == Hinge::Left
        {
            return 6631;
        }
        if self.r#facing == Facing::East
            && self.r#hinge == Hinge::Left
            && self.r#open == false
            && self.r#powered == true
            && self.r#half == Half::Upper
        {
            return 6646;
        }
        if self.r#hinge == Hinge::Right
            && self.r#open == false
            && self.r#half == Half::Lower
            && self.r#powered == false
            && self.r#facing == Facing::North
        {
            return 6611;
        }
        if self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#half == Half::Lower
            && self.r#open == true
            && self.r#hinge == Hinge::Left
        {
            return 6637;
        }
        if self.r#powered == false
            && self.r#half == Half::Upper
            && self.r#facing == Facing::North
            && self.r#open == false
            && self.r#hinge == Hinge::Left
        {
            return 6599;
        }
        if self.r#facing == Facing::North
            && self.r#hinge == Hinge::Right
            && self.r#open == true
            && self.r#powered == false
            && self.r#half == Half::Lower
        {
            return 6609;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Upper
            && self.r#powered == true
            && self.r#hinge == Hinge::Right
            && self.r#open == false
        {
            return 6634;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Lower
            && self.r#open == true
            && self.r#powered == true
            && self.r#hinge == Hinge::Right
        {
            return 6608;
        }
        if self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
            && self.r#open == false
        {
            return 6610;
        }
        if self.r#open == false
            && self.r#powered == false
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::South
            && self.r#half == Half::Upper
        {
            return 6615;
        }
        if self.r#half == Half::Upper
            && self.r#open == true
            && self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#hinge == Hinge::Right
        {
            return 6601;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Upper
            && self.r#open == false
            && self.r#hinge == Hinge::Left
            && self.r#powered == true
        {
            return 6614;
        }
        if self.r#powered == true
            && self.r#hinge == Hinge::Right
            && self.r#open == true
            && self.r#half == Half::Lower
            && self.r#facing == Facing::South
        {
            return 6624;
        }
        if self.r#powered == false
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::East
            && self.r#open == true
        {
            return 6653;
        }
        if self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#open == false
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Upper
        {
            return 6630;
        }
        if self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#hinge == Hinge::Right
            && self.r#open == true
            && self.r#half == Half::Upper
        {
            return 6617;
        }
        if self.r#hinge == Hinge::Left
            && self.r#half == Half::Lower
            && self.r#facing == Facing::West
            && self.r#open == false
            && self.r#powered == true
        {
            return 6638;
        }
        if self.r#hinge == Hinge::Right
            && self.r#open == false
            && self.r#powered == false
            && self.r#half == Half::Upper
            && self.r#facing == Facing::East
        {
            return 6651;
        }
        if self.r#hinge == Hinge::Left
            && self.r#half == Half::Upper
            && self.r#facing == Facing::South
            && self.r#open == true
            && self.r#powered == false
        {
            return 6613;
        }
        if self.r#half == Half::Upper
            && self.r#open == true
            && self.r#hinge == Hinge::Left
            && self.r#powered == false
            && self.r#facing == Facing::West
        {
            return 6629;
        }
        if self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::West
            && self.r#open == false
            && self.r#powered == false
        {
            return 6643;
        }
        if self.r#half == Half::Lower
            && self.r#facing == Facing::South
            && self.r#open == false
            && self.r#hinge == Hinge::Right
            && self.r#powered == false
        {
            return 6627;
        }
        if self.r#powered == true
            && self.r#facing == Facing::North
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Lower
            && self.r#open == false
        {
            return 6606;
        }
        if self.r#powered == true
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::East
            && self.r#half == Half::Lower
            && self.r#open == false
        {
            return 6654;
        }
        if self.r#half == Half::Upper
            && self.r#facing == Facing::North
            && self.r#hinge == Hinge::Left
            && self.r#open == true
            && self.r#powered == false
        {
            return 6597;
        }
        if self.r#facing == Facing::North
            && self.r#hinge == Hinge::Right
            && self.r#powered == true
            && self.r#open == true
            && self.r#half == Half::Upper
        {
            return 6600;
        }
        if self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::North
            && self.r#open == false
            && self.r#powered == false
        {
            return 6603;
        }
        if self.r#hinge == Hinge::Left
            && self.r#open == true
            && self.r#half == Half::Upper
            && self.r#powered == true
            && self.r#facing == Facing::East
        {
            return 6644;
        }
        if self.r#open == false
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::West
            && self.r#half == Half::Upper
            && self.r#powered == false
        {
            return 6635;
        }
        if self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#open == false
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
        {
            return 6642;
        }
        if self.r#hinge == Hinge::Right
            && self.r#powered == false
            && self.r#half == Half::Upper
            && self.r#facing == Facing::East
            && self.r#open == true
        {
            return 6649;
        }
        if self.r#open == false
            && self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Lower
        {
            return 6655;
        }
        if self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
            && self.r#open == false
        {
            return 6602;
        }
        if self.r#powered == true
            && self.r#half == Half::Upper
            && self.r#open == true
            && self.r#facing == Facing::South
            && self.r#hinge == Hinge::Left
        {
            return 6612;
        }
        if self.r#hinge == Hinge::Left
            && self.r#facing == Facing::North
            && self.r#half == Half::Upper
            && self.r#open == false
            && self.r#powered == true
        {
            return 6598;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Lower
            && self.r#open == false
            && self.r#hinge == Hinge::Left
            && self.r#powered == true
        {
            return 6622;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
            && self.r#open == false
            && self.r#powered == false
        {
            return 6659;
        }
        if self.r#powered == false
            && self.r#open == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
        {
            return 6633;
        }
        if self.r#facing == Facing::West
            && self.r#open == false
            && self.r#powered == false
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Left
        {
            return 6639;
        }
        if self.r#powered == true
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Lower
            && self.r#facing == Facing::South
            && self.r#open == true
        {
            return 6620;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Lower
            && self.r#open == true
            && self.r#powered == true
            && self.r#hinge == Hinge::Left
        {
            return 6652;
        }
        if self.r#open == false
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::South
            && self.r#powered == true
            && self.r#half == Half::Upper
        {
            return 6618;
        }
        if self.r#powered == true
            && self.r#open == true
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::West
        {
            return 6632;
        }
        if self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Lower
            && self.r#open == true
        {
            return 6640;
        }
        if self.r#open == true
            && self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Lower
        {
            return 6641;
        }
        if self.r#facing == Facing::North
            && self.r#open == true
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Lower
            && self.r#powered == false
        {
            return 6605;
        }
        if self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
            && self.r#open == true
            && self.r#powered == false
            && self.r#facing == Facing::East
        {
            return 6657;
        }
        if self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#half == Half::Upper
            && self.r#open == false
            && self.r#hinge == Hinge::Right
        {
            return 6650;
        }
        if self.r#powered == false
            && self.r#facing == Facing::South
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Lower
            && self.r#open == true
        {
            return 6621;
        }
        if self.r#half == Half::Upper
            && self.r#facing == Facing::North
            && self.r#open == true
            && self.r#powered == true
            && self.r#hinge == Hinge::Left
        {
            return 6596;
        }
        if self.r#open == false
            && self.r#hinge == Hinge::Left
            && self.r#powered == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Lower
        {
            return 6623;
        }
        if self.r#half == Half::Upper
            && self.r#powered == true
            && self.r#hinge == Hinge::Right
            && self.r#open == true
            && self.r#facing == Facing::East
        {
            return 6648;
        }
        if self.r#open == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Left
            && self.r#powered == true
        {
            return 6604;
        }
        if self.r#half == Half::Lower
            && self.r#powered == false
            && self.r#open == false
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::North
        {
            return 6607;
        }
        if self.r#hinge == Hinge::Left
            && self.r#facing == Facing::East
            && self.r#half == Half::Upper
            && self.r#powered == false
            && self.r#open == false
        {
            return 6647;
        }
        if self.r#powered == true
            && self.r#facing == Facing::South
            && self.r#open == true
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
        {
            return 6616;
        }
        if self.r#half == Half::Lower
            && self.r#open == true
            && self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#hinge == Hinge::Right
        {
            return 6625;
        }
        if self.r#half == Half::Lower
            && self.r#open == true
            && self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#hinge == Hinge::Right
        {
            return 6656;
        }
        if self.r#open == true
            && self.r#hinge == Hinge::Left
            && self.r#powered == false
            && self.r#facing == Facing::East
            && self.r#half == Half::Upper
        {
            return 6645;
        }
        if self.r#powered == false
            && self.r#facing == Facing::South
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Upper
            && self.r#open == false
        {
            return 6619;
        }
        if self.r#hinge == Hinge::Left
            && self.r#facing == Facing::West
            && self.r#powered == true
            && self.r#open == true
            && self.r#half == Half::Upper
        {
            return 6628;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6626 {
            return Some(IronDoor {
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 6658 {
            return Some(IronDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 6636 {
            return Some(IronDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#facing: Facing::West,
                r#open: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 6631 {
            return Some(IronDoor {
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 6646 {
            return Some(IronDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 6611 {
            return Some(IronDoor {
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Lower,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 6637 {
            return Some(IronDoor {
                r#facing: Facing::West,
                r#powered: false,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 6599 {
            return Some(IronDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 6609 {
            return Some(IronDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 6634 {
            return Some(IronDoor {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 6608 {
            return Some(IronDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 6610 {
            return Some(IronDoor {
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 6615 {
            return Some(IronDoor {
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#half: Half::Upper,
            });
        }
        if state_id == 6601 {
            return Some(IronDoor {
                r#half: Half::Upper,
                r#open: true,
                r#facing: Facing::North,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 6614 {
            return Some(IronDoor {
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#open: false,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 6624 {
            return Some(IronDoor {
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Lower,
                r#facing: Facing::South,
            });
        }
        if state_id == 6653 {
            return Some(IronDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 6630 {
            return Some(IronDoor {
                r#powered: true,
                r#facing: Facing::West,
                r#open: false,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
            });
        }
        if state_id == 6617 {
            return Some(IronDoor {
                r#facing: Facing::South,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 6638 {
            return Some(IronDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 6651 {
            return Some(IronDoor {
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::East,
            });
        }
        if state_id == 6613 {
            return Some(IronDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 6629 {
            return Some(IronDoor {
                r#half: Half::Upper,
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 6643 {
            return Some(IronDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 6627 {
            return Some(IronDoor {
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: false,
            });
        }
        if state_id == 6606 {
            return Some(IronDoor {
                r#powered: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 6654 {
            return Some(IronDoor {
                r#powered: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 6597 {
            return Some(IronDoor {
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 6600 {
            return Some(IronDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#open: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 6603 {
            return Some(IronDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 6644 {
            return Some(IronDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 6635 {
            return Some(IronDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#powered: false,
            });
        }
        if state_id == 6642 {
            return Some(IronDoor {
                r#facing: Facing::West,
                r#powered: true,
                r#open: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 6649 {
            return Some(IronDoor {
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 6655 {
            return Some(IronDoor {
                r#open: false,
                r#facing: Facing::East,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
            });
        }
        if state_id == 6602 {
            return Some(IronDoor {
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 6612 {
            return Some(IronDoor {
                r#powered: true,
                r#half: Half::Upper,
                r#open: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 6598 {
            return Some(IronDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 6622 {
            return Some(IronDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#open: false,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 6659 {
            return Some(IronDoor {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 6633 {
            return Some(IronDoor {
                r#powered: false,
                r#open: true,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 6639 {
            return Some(IronDoor {
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 6620 {
            return Some(IronDoor {
                r#powered: true,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 6652 {
            return Some(IronDoor {
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 6618 {
            return Some(IronDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 6632 {
            return Some(IronDoor {
                r#powered: true,
                r#open: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
            });
        }
        if state_id == 6640 {
            return Some(IronDoor {
                r#facing: Facing::West,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: true,
            });
        }
        if state_id == 6641 {
            return Some(IronDoor {
                r#open: true,
                r#facing: Facing::West,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
            });
        }
        if state_id == 6605 {
            return Some(IronDoor {
                r#facing: Facing::North,
                r#open: true,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 6657 {
            return Some(IronDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 6650 {
            return Some(IronDoor {
                r#facing: Facing::East,
                r#powered: true,
                r#half: Half::Upper,
                r#open: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 6621 {
            return Some(IronDoor {
                r#powered: false,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: true,
            });
        }
        if state_id == 6596 {
            return Some(IronDoor {
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 6623 {
            return Some(IronDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Lower,
            });
        }
        if state_id == 6648 {
            return Some(IronDoor {
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 6604 {
            return Some(IronDoor {
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 6607 {
            return Some(IronDoor {
                r#half: Half::Lower,
                r#powered: false,
                r#open: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 6647 {
            return Some(IronDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 6616 {
            return Some(IronDoor {
                r#powered: true,
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 6625 {
            return Some(IronDoor {
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::South,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 6656 {
            return Some(IronDoor {
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::East,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 6645 {
            return Some(IronDoor {
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Upper,
            });
        }
        if state_id == 6619 {
            return Some(IronDoor {
                r#powered: false,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#open: false,
            });
        }
        if state_id == 6628 {
            return Some(IronDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#powered: true,
                r#open: true,
                r#half: Half::Upper,
            });
        }
        return None;
    }
}
