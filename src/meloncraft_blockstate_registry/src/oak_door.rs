use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakDoor {
    pub r#hinge: Hinge,
    pub r#half: Half,
    pub r#facing: Facing,
    pub open: bool,
    pub powered: bool,
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

impl BlockState for OakDoor {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South
            && self.r#hinge == Hinge::Left
            && self.r#powered == true
            && self.r#open == false
            && self.r#half == Half::Lower
        {
            return 5480;
        }
        if self.r#powered == true
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::North
            && self.r#open == true
        {
            return 5466;
        }
        if self.r#open == true
            && self.r#powered == false
            && self.r#facing == Facing::East
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
        {
            return 5507;
        }
        if self.r#facing == Facing::West
            && self.r#open == true
            && self.r#half == Half::Upper
            && self.r#powered == true
            && self.r#hinge == Hinge::Right
        {
            return 5490;
        }
        if self.r#half == Half::Lower
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::South
            && self.r#powered == true
            && self.r#open == true
        {
            return 5478;
        }
        if self.r#powered == false
            && self.r#half == Half::Upper
            && self.r#facing == Facing::West
            && self.r#open == true
            && self.r#hinge == Hinge::Right
        {
            return 5491;
        }
        if self.r#half == Half::Upper
            && self.r#facing == Facing::East
            && self.r#hinge == Hinge::Right
            && self.r#open == false
            && self.r#powered == false
        {
            return 5509;
        }
        if self.r#half == Half::Upper
            && self.r#open == false
            && self.r#hinge == Hinge::Right
            && self.r#powered == false
            && self.r#facing == Facing::South
        {
            return 5477;
        }
        if self.r#open == false
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Upper
            && self.r#powered == true
            && self.r#facing == Facing::East
        {
            return 5504;
        }
        if self.r#hinge == Hinge::Right
            && self.r#facing == Facing::East
            && self.r#powered == true
            && self.r#half == Half::Upper
            && self.r#open == true
        {
            return 5506;
        }
        if self.r#powered == true
            && self.r#open == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
        {
            return 5458;
        }
        if self.r#open == false
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::West
            && self.r#powered == false
        {
            return 5497;
        }
        if self.r#facing == Facing::North
            && self.r#hinge == Hinge::Right
            && self.r#powered == true
            && self.r#half == Half::Upper
            && self.r#open == false
        {
            return 5460;
        }
        if self.r#open == false
            && self.r#powered == true
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::North
        {
            return 5464;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
            && self.r#powered == false
            && self.r#open == true
        {
            return 5467;
        }
        if self.r#facing == Facing::East
            && self.r#hinge == Hinge::Left
            && self.r#open == true
            && self.r#powered == false
            && self.r#half == Half::Lower
        {
            return 5511;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Upper
            && self.r#powered == true
            && self.r#hinge == Hinge::Right
            && self.r#open == true
        {
            return 5474;
        }
        if self.r#facing == Facing::East
            && self.r#powered == false
            && self.r#hinge == Hinge::Left
            && self.r#open == false
            && self.r#half == Half::Lower
        {
            return 5513;
        }
        if self.r#open == false
            && self.r#half == Half::Upper
            && self.r#facing == Facing::West
            && self.r#hinge == Hinge::Right
            && self.r#powered == true
        {
            return 5492;
        }
        if self.r#hinge == Hinge::Right
            && self.r#half == Half::Lower
            && self.r#open == true
            && self.r#facing == Facing::East
            && self.r#powered == false
        {
            return 5515;
        }
        if self.r#open == true
            && self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
        {
            return 5498;
        }
        if self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#open == true
        {
            return 5459;
        }
        if self.r#facing == Facing::North
            && self.r#open == false
            && self.r#powered == true
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Left
        {
            return 5456;
        }
        if self.r#open == false
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::South
            && self.r#powered == false
            && self.r#half == Half::Lower
        {
            return 5481;
        }
        if self.r#hinge == Hinge::Left
            && self.r#facing == Facing::East
            && self.r#open == true
            && self.r#half == Half::Upper
            && self.r#powered == false
        {
            return 5503;
        }
        if self.r#open == false
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::West
            && self.r#powered == true
        {
            return 5488;
        }
        if self.r#hinge == Hinge::Left
            && self.r#open == false
            && self.r#half == Half::Upper
            && self.r#facing == Facing::South
            && self.r#powered == false
        {
            return 5473;
        }
        if self.r#facing == Facing::East
            && self.r#open == false
            && self.r#hinge == Hinge::Left
            && self.r#powered == true
            && self.r#half == Half::Lower
        {
            return 5512;
        }
        if self.r#hinge == Hinge::Left
            && self.r#powered == false
            && self.r#facing == Facing::East
            && self.r#half == Half::Upper
            && self.r#open == false
        {
            return 5505;
        }
        if self.r#half == Half::Lower
            && self.r#powered == false
            && self.r#open == true
            && self.r#hinge == Hinge::Left
            && self.r#facing == Facing::West
        {
            return 5495;
        }
        if self.r#half == Half::Upper
            && self.r#facing == Facing::West
            && self.r#hinge == Hinge::Right
            && self.r#open == false
            && self.r#powered == false
        {
            return 5493;
        }
        if self.r#half == Half::Upper
            && self.r#powered == false
            && self.r#facing == Facing::North
            && self.r#hinge == Hinge::Right
            && self.r#open == false
        {
            return 5461;
        }
        if self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#hinge == Hinge::Left
            && self.r#open == false
            && self.r#half == Half::Lower
        {
            return 5496;
        }
        if self.r#half == Half::Lower
            && self.r#powered == false
            && self.r#open == false
            && self.r#facing == Facing::North
            && self.r#hinge == Hinge::Right
        {
            return 5469;
        }
        if self.r#hinge == Hinge::Left
            && self.r#open == true
            && self.r#powered == true
            && self.r#half == Half::Lower
            && self.r#facing == Facing::East
        {
            return 5510;
        }
        if self.r#hinge == Hinge::Left
            && self.r#powered == false
            && self.r#half == Half::Lower
            && self.r#facing == Facing::North
            && self.r#open == true
        {
            return 5463;
        }
        if self.r#half == Half::Upper
            && self.r#facing == Facing::West
            && self.r#open == true
            && self.r#hinge == Hinge::Left
            && self.r#powered == true
        {
            return 5486;
        }
        if self.r#half == Half::Lower
            && self.r#hinge == Hinge::Left
            && self.r#open == true
            && self.r#facing == Facing::North
            && self.r#powered == true
        {
            return 5462;
        }
        if self.r#facing == Facing::North
            && self.r#powered == false
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Lower
            && self.r#open == false
        {
            return 5465;
        }
        if self.r#powered == true
            && self.r#open == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
        {
            return 5482;
        }
        if self.r#hinge == Hinge::Right
            && self.r#powered == false
            && self.r#half == Half::Lower
            && self.r#facing == Facing::West
            && self.r#open == true
        {
            return 5499;
        }
        if self.r#half == Half::Upper
            && self.r#open == true
            && self.r#facing == Facing::South
            && self.r#hinge == Hinge::Left
            && self.r#powered == false
        {
            return 5471;
        }
        if self.r#powered == true
            && self.r#open == true
            && self.r#facing == Facing::East
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Upper
        {
            return 5502;
        }
        if self.r#open == false
            && self.r#half == Half::Lower
            && self.r#facing == Facing::North
            && self.r#powered == true
            && self.r#hinge == Hinge::Right
        {
            return 5468;
        }
        if self.r#half == Half::Lower
            && self.r#facing == Facing::West
            && self.r#powered == false
            && self.r#hinge == Hinge::Right
            && self.r#open == false
        {
            return 5501;
        }
        if self.r#powered == true
            && self.r#open == false
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::East
            && self.r#half == Half::Lower
        {
            return 5516;
        }
        if self.r#powered == false
            && self.r#hinge == Hinge::Left
            && self.r#open == false
            && self.r#facing == Facing::North
            && self.r#half == Half::Upper
        {
            return 5457;
        }
        if self.r#facing == Facing::West
            && self.r#hinge == Hinge::Left
            && self.r#open == true
            && self.r#powered == false
            && self.r#half == Half::Upper
        {
            return 5487;
        }
        if self.r#half == Half::Lower
            && self.r#facing == Facing::West
            && self.r#open == false
            && self.r#powered == true
            && self.r#hinge == Hinge::Right
        {
            return 5500;
        }
        if self.r#open == true
            && self.r#facing == Facing::North
            && self.r#hinge == Hinge::Left
            && self.r#powered == true
            && self.r#half == Half::Upper
        {
            return 5454;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Lower
            && self.r#powered == false
            && self.r#hinge == Hinge::Right
            && self.r#open == false
        {
            return 5485;
        }
        if self.r#facing == Facing::South
            && self.r#powered == true
            && self.r#hinge == Hinge::Right
            && self.r#half == Half::Lower
            && self.r#open == false
        {
            return 5484;
        }
        if self.r#open == true
            && self.r#facing == Facing::South
            && self.r#hinge == Hinge::Left
            && self.r#half == Half::Lower
            && self.r#powered == false
        {
            return 5479;
        }
        if self.r#half == Half::Upper
            && self.r#powered == true
            && self.r#open == false
            && self.r#facing == Facing::South
            && self.r#hinge == Hinge::Right
        {
            return 5476;
        }
        if self.r#facing == Facing::East
            && self.r#open == true
            && self.r#powered == true
            && self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
        {
            return 5514;
        }
        if self.r#half == Half::Lower
            && self.r#hinge == Hinge::Right
            && self.r#facing == Facing::South
            && self.r#open == true
            && self.r#powered == false
        {
            return 5483;
        }
        if self.r#open == true
            && self.r#hinge == Hinge::Left
            && self.r#powered == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Lower
        {
            return 5494;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Upper
            && self.r#powered == true
            && self.r#hinge == Hinge::Left
            && self.r#open == false
        {
            return 5472;
        }
        if self.r#half == Half::Upper
            && self.r#facing == Facing::East
            && self.r#hinge == Hinge::Right
            && self.r#open == false
            && self.r#powered == true
        {
            return 5508;
        }
        if self.r#powered == false
            && self.r#facing == Facing::South
            && self.r#open == true
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Right
        {
            return 5475;
        }
        if self.r#half == Half::Lower
            && self.r#facing == Facing::East
            && self.r#hinge == Hinge::Right
            && self.r#open == false
            && self.r#powered == false
        {
            return 5517;
        }
        if self.r#half == Half::Upper
            && self.r#open == true
            && self.r#facing == Facing::North
            && self.r#hinge == Hinge::Left
            && self.r#powered == false
        {
            return 5455;
        }
        if self.r#open == false
            && self.r#powered == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Upper
            && self.r#hinge == Hinge::Left
        {
            return 5489;
        }
        if self.r#facing == Facing::South
            && self.r#hinge == Hinge::Left
            && self.r#open == true
            && self.r#powered == true
            && self.r#half == Half::Upper
        {
            return 5470;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5480 {
            return Some(OakDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#open: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 5466 {
            return Some(OakDoor {
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 5507 {
            return Some(OakDoor {
                r#open: true,
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 5490 {
            return Some(OakDoor {
                r#facing: Facing::West,
                r#open: true,
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 5478 {
            return Some(OakDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 5491 {
            return Some(OakDoor {
                r#powered: false,
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#open: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 5509 {
            return Some(OakDoor {
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 5477 {
            return Some(OakDoor {
                r#half: Half::Upper,
                r#open: false,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 5504 {
            return Some(OakDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 5506 {
            return Some(OakDoor {
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#powered: true,
                r#half: Half::Upper,
                r#open: true,
            });
        }
        if state_id == 5458 {
            return Some(OakDoor {
                r#powered: true,
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 5497 {
            return Some(OakDoor {
                r#open: false,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 5460 {
            return Some(OakDoor {
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#powered: true,
                r#half: Half::Upper,
                r#open: false,
            });
        }
        if state_id == 5464 {
            return Some(OakDoor {
                r#open: false,
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#facing: Facing::North,
            });
        }
        if state_id == 5467 {
            return Some(OakDoor {
                r#facing: Facing::North,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 5511 {
            return Some(OakDoor {
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 5474 {
            return Some(OakDoor {
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#open: true,
            });
        }
        if state_id == 5513 {
            return Some(OakDoor {
                r#facing: Facing::East,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#open: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 5492 {
            return Some(OakDoor {
                r#open: false,
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#powered: true,
            });
        }
        if state_id == 5515 {
            return Some(OakDoor {
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: true,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 5498 {
            return Some(OakDoor {
                r#open: true,
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 5459 {
            return Some(OakDoor {
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
                r#facing: Facing::North,
                r#powered: false,
                r#open: true,
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
        if state_id == 5481 {
            return Some(OakDoor {
                r#open: false,
                r#hinge: Hinge::Left,
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 5503 {
            return Some(OakDoor {
                r#hinge: Hinge::Left,
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Upper,
                r#powered: false,
            });
        }
        if state_id == 5488 {
            return Some(OakDoor {
                r#open: false,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 5473 {
            return Some(OakDoor {
                r#hinge: Hinge::Left,
                r#open: false,
                r#half: Half::Upper,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 5512 {
            return Some(OakDoor {
                r#facing: Facing::East,
                r#open: false,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Lower,
            });
        }
        if state_id == 5505 {
            return Some(OakDoor {
                r#hinge: Hinge::Left,
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Upper,
                r#open: false,
            });
        }
        if state_id == 5495 {
            return Some(OakDoor {
                r#half: Half::Lower,
                r#powered: false,
                r#open: true,
                r#hinge: Hinge::Left,
                r#facing: Facing::West,
            });
        }
        if state_id == 5493 {
            return Some(OakDoor {
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 5461 {
            return Some(OakDoor {
                r#half: Half::Upper,
                r#powered: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 5496 {
            return Some(OakDoor {
                r#powered: true,
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#open: false,
                r#half: Half::Lower,
            });
        }
        if state_id == 5469 {
            return Some(OakDoor {
                r#half: Half::Lower,
                r#powered: false,
                r#open: false,
                r#facing: Facing::North,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 5510 {
            return Some(OakDoor {
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
                r#facing: Facing::East,
            });
        }
        if state_id == 5463 {
            return Some(OakDoor {
                r#hinge: Hinge::Left,
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 5486 {
            return Some(OakDoor {
                r#half: Half::Upper,
                r#facing: Facing::West,
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: true,
            });
        }
        if state_id == 5462 {
            return Some(OakDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Left,
                r#open: true,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 5465 {
            return Some(OakDoor {
                r#facing: Facing::North,
                r#powered: false,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 5482 {
            return Some(OakDoor {
                r#powered: true,
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 5499 {
            return Some(OakDoor {
                r#hinge: Hinge::Right,
                r#powered: false,
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 5471 {
            return Some(OakDoor {
                r#half: Half::Upper,
                r#open: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 5502 {
            return Some(OakDoor {
                r#powered: true,
                r#open: true,
                r#facing: Facing::East,
                r#hinge: Hinge::Left,
                r#half: Half::Upper,
            });
        }
        if state_id == 5468 {
            return Some(OakDoor {
                r#open: false,
                r#half: Half::Lower,
                r#facing: Facing::North,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 5501 {
            return Some(OakDoor {
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 5516 {
            return Some(OakDoor {
                r#powered: true,
                r#open: false,
                r#hinge: Hinge::Right,
                r#facing: Facing::East,
                r#half: Half::Lower,
            });
        }
        if state_id == 5457 {
            return Some(OakDoor {
                r#powered: false,
                r#hinge: Hinge::Left,
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Upper,
            });
        }
        if state_id == 5487 {
            return Some(OakDoor {
                r#facing: Facing::West,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: false,
                r#half: Half::Upper,
            });
        }
        if state_id == 5500 {
            return Some(OakDoor {
                r#half: Half::Lower,
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 5454 {
            return Some(OakDoor {
                r#open: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        if state_id == 5485 {
            return Some(OakDoor {
                r#facing: Facing::South,
                r#half: Half::Lower,
                r#powered: false,
                r#hinge: Hinge::Right,
                r#open: false,
            });
        }
        if state_id == 5484 {
            return Some(OakDoor {
                r#facing: Facing::South,
                r#powered: true,
                r#hinge: Hinge::Right,
                r#half: Half::Lower,
                r#open: false,
            });
        }
        if state_id == 5479 {
            return Some(OakDoor {
                r#open: true,
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#half: Half::Lower,
                r#powered: false,
            });
        }
        if state_id == 5476 {
            return Some(OakDoor {
                r#half: Half::Upper,
                r#powered: true,
                r#open: false,
                r#facing: Facing::South,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 5514 {
            return Some(OakDoor {
                r#facing: Facing::East,
                r#open: true,
                r#powered: true,
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 5483 {
            return Some(OakDoor {
                r#half: Half::Lower,
                r#hinge: Hinge::Right,
                r#facing: Facing::South,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 5494 {
            return Some(OakDoor {
                r#open: true,
                r#hinge: Hinge::Left,
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Lower,
            });
        }
        if state_id == 5472 {
            return Some(OakDoor {
                r#facing: Facing::South,
                r#half: Half::Upper,
                r#powered: true,
                r#hinge: Hinge::Left,
                r#open: false,
            });
        }
        if state_id == 5508 {
            return Some(OakDoor {
                r#half: Half::Upper,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 5475 {
            return Some(OakDoor {
                r#powered: false,
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Upper,
                r#hinge: Hinge::Right,
            });
        }
        if state_id == 5517 {
            return Some(OakDoor {
                r#half: Half::Lower,
                r#facing: Facing::East,
                r#hinge: Hinge::Right,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 5455 {
            return Some(OakDoor {
                r#half: Half::Upper,
                r#open: true,
                r#facing: Facing::North,
                r#hinge: Hinge::Left,
                r#powered: false,
            });
        }
        if state_id == 5489 {
            return Some(OakDoor {
                r#open: false,
                r#powered: false,
                r#facing: Facing::West,
                r#half: Half::Upper,
                r#hinge: Hinge::Left,
            });
        }
        if state_id == 5470 {
            return Some(OakDoor {
                r#facing: Facing::South,
                r#hinge: Hinge::Left,
                r#open: true,
                r#powered: true,
                r#half: Half::Upper,
            });
        }
        return None;
    }
}
