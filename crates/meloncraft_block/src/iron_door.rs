use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IronDoor {
    pub r#half: Half,
    pub r#hinge: Hinge,
    pub powered: bool,
    pub open: bool,
    pub r#facing: Facing,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for IronDoor {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right { return 6651; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#open == false { return 6598; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#half == Half::Upper && block_state.r#powered == true { return 6646; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower { return 6627; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#open == true { return 6637; }
        if block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#facing == Facing::West && block_state.r#open == true { return 6633; }
        if block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#open == true { return 6612; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right { return 6616; }
        if block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left { return 6629; }
        if block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::North { return 6611; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#powered == false { return 6631; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::West { return 6640; }
        if block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#powered == true { return 6626; }
        if block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::North && block_state.r#powered == true { return 6610; }
        if block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == false && block_state.r#half == Half::Lower { return 6607; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#powered == true { return 6634; }
        if block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right { return 6642; }
        if block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#powered == true { return 6650; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#powered == false { return 6659; }
        if block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#powered == false { return 6603; }
        if block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#powered == false { return 6657; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#open == true { return 6609; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == true { return 6604; }
        if block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#open == false { return 6638; }
        if block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left { return 6606; }
        if block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left { return 6614; }
        if block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right { return 6624; }
        if block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper { return 6600; }
        if block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#open == false && block_state.r#half == Half::Lower { return 6639; }
        if block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#open == true { return 6645; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#facing == Facing::West { return 6636; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#hinge == Hinge::Left { return 6655; }
        if block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right { return 6625; }
        if block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#half == Half::Lower && block_state.r#open == true { return 6605; }
        if block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right { return 6601; }
        if block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::North && block_state.r#powered == true { return 6602; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Left { return 6621; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#open == false { return 6647; }
        if block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right { return 6658; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == true { return 6630; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#open == true { return 6620; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#half == Half::Upper { return 6635; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#half == Half::Lower { return 6656; }
        if block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#half == Half::Upper && block_state.r#facing == Facing::South { return 6615; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#powered == true { return 6622; }
        if block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::South { return 6617; }
        if block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#hinge == Hinge::Right { return 6649; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Lower { return 6652; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#half == Half::Upper { return 6619; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right { return 6641; }
        if block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#hinge == Hinge::Left { return 6644; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#facing == Facing::South { return 6613; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#half == Half::Lower { return 6653; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower { return 6623; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#half == Half::Upper { return 6628; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#open == false { return 6618; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#powered == false && block_state.r#hinge == Hinge::Left { return 6599; }
        if block_state.r#powered == false && block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper { return 6597; }
        if block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower { return 6608; }
        if block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#facing == Facing::West { return 6643; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#open == false && block_state.r#hinge == Hinge::Left { return 6654; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Upper { return 6596; }
        if block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#powered == true { return 6632; }
        if block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Upper { return 6648; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6651 {
            return Some(IronDoor {
                r#facing: Facing::East,
                r#open: false,
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 6598 {
            return Some(IronDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 6646 {
            return Some(IronDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 6627 {
            return Some(IronDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Lower,
            });
        }
        if state_id == 6637 {
            return Some(IronDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 6633 {
            return Some(IronDoor {
                r#powered: false,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 6612 {
            return Some(IronDoor {
                r#facing: Facing::South,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: true,
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
        if state_id == 6629 {
            return Some(IronDoor {
                r#open: true,
                r#facing: Facing::West,
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 6611 {
            return Some(IronDoor {
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 6631 {
            return Some(IronDoor {
                r#hinge: Hinge::Left,
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#powered: false,
            });
        }
        if state_id == 6640 {
            return Some(IronDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 6626 {
            return Some(IronDoor {
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#powered: true,
            });
        }
        if state_id == 6610 {
            return Some(IronDoor {
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 6607 {
            return Some(IronDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#open: false,
                r#powered: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 6634 {
            return Some(IronDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 6642 {
            return Some(IronDoor {
                r#half: Half::Lower,
                r#powered: true,
                r#open: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 6650 {
            return Some(IronDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 6659 {
            return Some(IronDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 6603 {
            return Some(IronDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 6657 {
            return Some(IronDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 6609 {
            return Some(IronDoor {
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 6604 {
            return Some(IronDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 6638 {
            return Some(IronDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 6606 {
            return Some(IronDoor {
                r#half: Half::Lower,
                r#open: false,
                r#powered: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 6614 {
            return Some(IronDoor {
                r#open: false,
                r#facing: Facing::South,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 6624 {
            return Some(IronDoor {
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 6600 {
            return Some(IronDoor {
                r#open: true,
                r#facing: Facing::North,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
            });
        }
        if state_id == 6639 {
            return Some(IronDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#powered: false,
                r#open: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 6645 {
            return Some(IronDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 6636 {
            return Some(IronDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 6655 {
            return Some(IronDoor {
                r#facing: Facing::East,
                r#powered: false,
                r#half: Half::Lower,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 6625 {
            return Some(IronDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 6605 {
            return Some(IronDoor {
                r#powered: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#open: true,
            });
        }
        if state_id == 6601 {
            return Some(IronDoor {
                r#facing: Facing::North,
                r#open: true,
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 6602 {
            return Some(IronDoor {
                r#open: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 6621 {
            return Some(IronDoor {
                r#powered: false,
                r#open: true,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 6647 {
            return Some(IronDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 6658 {
            return Some(IronDoor {
                r#powered: true,
                r#half: Half::Lower,
                r#open: false,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 6630 {
            return Some(IronDoor {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 6620 {
            return Some(IronDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 6635 {
            return Some(IronDoor {
                r#powered: false,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 6656 {
            return Some(IronDoor {
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 6615 {
            return Some(IronDoor {
                r#powered: false,
                r#hinge: Hinge::Left,
                r#open: false,
                r#half: Half::Upper,
                r#facing: Facing::South,
            });
        }
        if state_id == 6622 {
            return Some(IronDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#powered: true,
            });
        }
        if state_id == 6617 {
            return Some(IronDoor {
                r#powered: false,
                r#hinge: Hinge::Right,
                r#open: true,
                r#half: Half::Upper,
                r#facing: Facing::South,
            });
        }
        if state_id == 6649 {
            return Some(IronDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 6652 {
            return Some(IronDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Lower,
            });
        }
        if state_id == 6619 {
            return Some(IronDoor {
                r#powered: false,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#open: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 6641 {
            return Some(IronDoor {
                r#facing: Facing::West,
                r#powered: false,
                r#open: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 6644 {
            return Some(IronDoor {
                r#open: true,
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 6613 {
            return Some(IronDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::South,
            });
        }
        if state_id == 6653 {
            return Some(IronDoor {
                r#hinge: Hinge::Left,
                r#powered: false,
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 6623 {
            return Some(IronDoor {
                r#hinge: Hinge::Left,
                r#open: false,
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Lower,
            });
        }
        if state_id == 6628 {
            return Some(IronDoor {
                r#powered: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#open: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 6618 {
            return Some(IronDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 6599 {
            return Some(IronDoor {
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 6597 {
            return Some(IronDoor {
                r#powered: false,
                r#hinge: Hinge::Left,
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Upper,
            });
        }
        if state_id == 6608 {
            return Some(IronDoor {
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
            });
        }
        if state_id == 6643 {
            return Some(IronDoor {
                r#open: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 6654 {
            return Some(IronDoor {
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 6596 {
            return Some(IronDoor {
                r#hinge: Hinge::Left,
                r#powered: true,
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Upper,
            });
        }
        if state_id == 6632 {
            return Some(IronDoor {
                r#hinge: Hinge::Right,
                r#open: true,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 6648 {
            return Some(IronDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        return None;
    }
}

