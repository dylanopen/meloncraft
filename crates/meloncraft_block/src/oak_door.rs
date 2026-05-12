use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakDoor {
    pub r#facing: Facing,
    pub r#half: Half,
    pub powered: bool,
    pub r#hinge: Hinge,
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
    Upper,
    Lower,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hinge {
    Left,
    Right,
}

impl BlockState for OakDoor {
    fn to_id(self) -> i32 {
        if block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#powered == false { return 5493; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#open == true { return 5494; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#powered == false { return 5491; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left { return 5496; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Upper { return 5508; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::East { return 5509; }
        if block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left { return 5456; }
        if block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#half == Half::Lower { return 5510; }
        if block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::North { return 5468; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == false { return 5472; }
        if block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#powered == true { return 5498; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#open == false && block_state.r#hinge == Hinge::Left { return 5505; }
        if block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#powered == false { return 5471; }
        if block_state.r#half == Half::Upper && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Left { return 5457; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == true { return 5464; }
        if block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right { return 5461; }
        if block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper { return 5504; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#open == false { return 5517; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == true { return 5466; }
        if block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North { return 5465; }
        if block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#powered == false { return 5487; }
        if block_state.r#open == true && block_state.r#half == Half::Upper && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#hinge == Hinge::Left { return 5454; }
        if block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower { return 5479; }
        if block_state.r#powered == true && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right { return 5492; }
        if block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#powered == false { return 5499; }
        if block_state.r#powered == true && block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right { return 5500; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#open == true && block_state.r#hinge == Hinge::Right { return 5475; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#powered == true && block_state.r#hinge == Hinge::Left { return 5480; }
        if block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left && block_state.r#powered == true { return 5512; }
        if block_state.r#powered == true && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East { return 5514; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Left && block_state.r#open == false && block_state.r#half == Half::Upper { return 5488; }
        if block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#powered == true { return 5458; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#facing == Facing::North && block_state.r#powered == false { return 5463; }
        if block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#powered == false { return 5473; }
        if block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#powered == true && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::South { return 5478; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#powered == true && block_state.r#facing == Facing::South { return 5484; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == false { return 5507; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#facing == Facing::East { return 5513; }
        if block_state.r#open == false && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Upper && block_state.r#facing == Facing::North && block_state.r#powered == true { return 5460; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower { return 5469; }
        if block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Upper && block_state.r#powered == true { return 5486; }
        if block_state.r#hinge == Hinge::Left && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#half == Half::Lower { return 5462; }
        if block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Left && block_state.r#open == false { return 5481; }
        if block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#powered == false { return 5477; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Right && block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::North { return 5459; }
        if block_state.r#facing == Facing::East && block_state.r#hinge == Hinge::Left && block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#open == true { return 5511; }
        if block_state.r#hinge == Hinge::Right && block_state.r#half == Half::Lower && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#open == true { return 5483; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#half == Half::Upper { return 5506; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#facing == Facing::North { return 5455; }
        if block_state.r#half == Half::Upper && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#hinge == Hinge::Right && block_state.r#open == true { return 5490; }
        if block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::East { return 5502; }
        if block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#powered == false && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::East { return 5515; }
        if block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#facing == Facing::North && block_state.r#hinge == Hinge::Right && block_state.r#open == true { return 5467; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Upper && block_state.r#powered == false && block_state.r#open == false && block_state.r#hinge == Hinge::Left { return 5489; }
        if block_state.r#hinge == Hinge::Left && block_state.r#powered == false && block_state.r#half == Half::Lower && block_state.r#open == true && block_state.r#facing == Facing::West { return 5495; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Lower && block_state.r#open == false && block_state.r#powered == false && block_state.r#hinge == Hinge::Right { return 5501; }
        if block_state.r#half == Half::Upper && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right && block_state.r#open == true && block_state.r#powered == true { return 5474; }
        if block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#open == false { return 5516; }
        if block_state.r#open == true && block_state.r#half == Half::Lower && block_state.r#hinge == Hinge::Right && block_state.r#facing == Facing::South && block_state.r#powered == true { return 5482; }
        if block_state.r#powered == true && block_state.r#hinge == Hinge::Right && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper { return 5476; }
        if block_state.r#half == Half::Lower && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == false && block_state.r#hinge == Hinge::Left { return 5497; }
        if block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#powered == true { return 5470; }
        if block_state.r#open == false && block_state.r#half == Half::Lower && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#hinge == Hinge::Right { return 5485; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Upper && block_state.r#hinge == Hinge::Left && block_state.r#open == true { return 5503; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5493 {
            return Some(OakDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#powered: false,
            });
        }
        if state_id == 5494 {
            return Some(OakDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 5491 {
            return Some(OakDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: true,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 5496 {
            return Some(OakDoor {
                r#open: false,
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 5508 {
            return Some(OakDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Upper,
            });
        }
        if state_id == 5509 {
            return Some(OakDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#powered: false,
                r#open: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 5456 {
            return Some(OakDoor {
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 5510 {
            return Some(OakDoor {
                r#open: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 5468 {
            return Some(OakDoor {
                r#half: Half::Lower,
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
            });
        }
        if state_id == 5472 {
            return Some(OakDoor {
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 5498 {
            return Some(OakDoor {
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#powered: true,
            });
        }
        if state_id == 5505 {
            return Some(OakDoor {
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#powered: false,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 5471 {
            return Some(OakDoor {
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#open: true,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 5457 {
            return Some(OakDoor {
                r#half: Half::Upper,
                r#open: false,
                r#powered: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 5464 {
            return Some(OakDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 5461 {
            return Some(OakDoor {
                r#half: Half::Upper,
                r#powered: false,
                r#open: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 5504 {
            return Some(OakDoor {
                r#facing: Facing::East,
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
            });
        }
        if state_id == 5517 {
            return Some(OakDoor {
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 5466 {
            return Some(OakDoor {
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 5465 {
            return Some(OakDoor {
                r#half: Half::Lower,
                r#powered: false,
                r#open: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 5487 {
            return Some(OakDoor {
                r#open: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#powered: false,
            });
        }
        if state_id == 5454 {
            return Some(OakDoor {
                r#open: true,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 5479 {
            return Some(OakDoor {
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Lower,
            });
        }
        if state_id == 5492 {
            return Some(OakDoor {
                r#powered: true,
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 5499 {
            return Some(OakDoor {
                r#hinge: Hinge::Right,
                r#open: true,
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 5500 {
            return Some(OakDoor {
                r#powered: true,
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 5475 {
            return Some(OakDoor {
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#powered: false,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 5480 {
            return Some(OakDoor {
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 5512 {
            return Some(OakDoor {
                r#half: Half::Lower,
                r#open: false,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 5514 {
            return Some(OakDoor {
                r#powered: true,
                r#half: Half::Lower,
                r#open: true,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 5488 {
            return Some(OakDoor {
                r#powered: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#open: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 5458 {
            return Some(OakDoor {
                r#facing: Facing::North,
                r#open: true,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 5463 {
            return Some(OakDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 5473 {
            return Some(OakDoor {
                r#facing: Facing::South,
                r#open: false,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#powered: false,
            });
        }
        if state_id == 5478 {
            return Some(OakDoor {
                r#half: Half::Lower,
                r#open: true,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
            });
        }
        if state_id == 5484 {
            return Some(OakDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 5507 {
            return Some(OakDoor {
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 5513 {
            return Some(OakDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#powered: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 5460 {
            return Some(OakDoor {
                r#open: false,
                r#hinge: Hinge::Right,
                r#half: Half::Upper,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 5469 {
            return Some(OakDoor {
                r#open: false,
                r#powered: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
            });
        }
        if state_id == 5486 {
            return Some(OakDoor {
                r#facing: Facing::West,
                r#open: true,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#powered: true,
            });
        }
        if state_id == 5462 {
            return Some(OakDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 5481 {
            return Some(OakDoor {
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 5477 {
            return Some(OakDoor {
                r#hinge: Hinge::Right,
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#powered: false,
            });
        }
        if state_id == 5459 {
            return Some(OakDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#open: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 5511 {
            return Some(OakDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 5483 {
            return Some(OakDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#facing: Facing::South,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 5506 {
            return Some(OakDoor {
                r#facing: Facing::East,
                r#open: true,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 5455 {
            return Some(OakDoor {
                r#open: true,
                r#powered: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 5490 {
            return Some(OakDoor {
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 5502 {
            return Some(OakDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#open: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 5515 {
            return Some(OakDoor {
                r#half: Half::Lower,
                r#open: true,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
            });
        }
        if state_id == 5467 {
            return Some(OakDoor {
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 5489 {
            return Some(OakDoor {
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#powered: false,
                r#open: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 5495 {
            return Some(OakDoor {
                r#hinge: Hinge::Left,
                r#powered: false,
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 5501 {
            return Some(OakDoor {
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 5474 {
            return Some(OakDoor {
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 5516 {
            return Some(OakDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 5482 {
            return Some(OakDoor {
                r#open: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 5476 {
            return Some(OakDoor {
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Upper,
            });
        }
        if state_id == 5497 {
            return Some(OakDoor {
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 5470 {
            return Some(OakDoor {
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 5485 {
            return Some(OakDoor {
                r#open: false,
                r#half: Half::Lower,
                r#powered: false,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 5503 {
            return Some(OakDoor {
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#open: true,
            });
        }
        return None;
    }
}

