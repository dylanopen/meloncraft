use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedOxidizedCutCopperStairs {
    pub r#shape: Shape,
    pub r#facing: Facing,
    pub r#half: Half,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Straight,
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight,
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

impl BlockState for WaxedOxidizedCutCopperStairs {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South && self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#waterlogged == true { return 25497; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::InnerRight { return 25522; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#waterlogged == false { return 25536; }
        if self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#facing == Facing::North && self.r#waterlogged == false { return 25478; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom { return 25512; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#facing == Facing::South { return 25516; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 25534; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#half == Half::Top { return 25538; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#facing == Facing::East { return 25548; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 25510; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == true { return 25521; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft { return 25550; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Bottom { return 25554; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#shape == Shape::OuterRight { return 25555; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft { return 25519; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight { return 25551; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 25556; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#facing == Facing::South { return 25514; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom { return 25494; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#waterlogged == true { return 25487; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#facing == Facing::North { return 25495; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#facing == Facing::West { return 25525; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft { return 25524; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == false { return 25490; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::OuterRight { return 25545; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::West { return 25523; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#facing == Facing::South { return 25506; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 25527; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#facing == Facing::South { return 25505; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Top { return 25537; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#half == Half::Top { return 25502; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 25489; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft { return 25539; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft { return 25544; }
        if self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::East { return 25549; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::South { return 25508; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#half == Half::Top { return 25481; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#shape == Shape::Straight { return 25477; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == true { return 25491; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#facing == Facing::North { return 25486; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::South { return 25504; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::Straight { return 25528; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#facing == Facing::North { return 25483; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft { return 25530; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::InnerRight { return 25531; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 25541; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top { return 25543; }
        if self.r#facing == Facing::South && self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#waterlogged == true { return 25515; }
        if self.r#waterlogged == true && self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#facing == Facing::South { return 25507; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#waterlogged == false { return 25484; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 25540; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::InnerRight && self.r#waterlogged == false { return 25482; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 25529; }
        if self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#facing == Facing::West && self.r#waterlogged == false { return 25520; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 25488; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom { return 25553; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == true { return 25499; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 25493; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::InnerRight { return 25532; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::North { return 25480; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft { return 25479; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight { return 25492; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#waterlogged == true { return 25511; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::OuterRight { return 25535; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#facing == Facing::East { return 25547; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#facing == Facing::East { return 25546; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 25485; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#facing == Facing::East && self.r#waterlogged == false { return 25542; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft { return 25503; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom { return 25552; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South && self.r#half == Half::Bottom { return 25513; }
        if self.r#waterlogged == true && self.r#shape == Shape::Straight && self.r#facing == Facing::West && self.r#half == Half::Top { return 25517; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 25501; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 25496; }
        if self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#facing == Facing::South && self.r#waterlogged == false { return 25498; }
        if self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#half == Half::Top { return 25500; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 25533; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Top { return 25518; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == false { return 25526; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#half == Half::Bottom { return 25509; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25497 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 25522 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25536 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 25478 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 25512 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25516 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
            });
        }
        if state_id == 25534 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25538 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 25548 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
            });
        }
        if state_id == 25510 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 25521 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 25550 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25554 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25555 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25519 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25551 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25556 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 25514 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 25494 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25487 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 25495 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 25525 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 25524 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25490 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 25545 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25523 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 25506 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 25527 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 25505 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 25537 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 25502 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 25489 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 25539 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25544 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25549 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 25508 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 25481 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 25477 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25491 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25486 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
            });
        }
        if state_id == 25504 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 25528 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25483 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 25530 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25531 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25541 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 25543 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 25515 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25507 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 25484 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 25540 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 25482 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 25529 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 25520 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 25488 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25553 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25499 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 25493 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25532 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25480 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 25479 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25492 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25511 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 25535 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25547 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 25546 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 25485 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 25542 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 25503 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25552 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25513 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25517 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 25501 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25496 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25498 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 25500 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 25533 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 25518 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 25526 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25509 {
            return Some(WaxedOxidizedCutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        return None;
    }
}

