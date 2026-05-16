use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedBlackstoneStairs {
    pub r#half: Half,
    pub r#facing: Facing,
    pub waterlogged: bool,
    pub r#shape: Shape,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Straight,
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight,
}

impl BlockState for PolishedBlackstoneStairs {
    fn to_id(&self) -> i32 {
        if self.r#shape == Shape::Straight && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == false { return 22516; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft { return 22501; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::OuterRight { return 22524; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 22532; }
        if self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#waterlogged == true { return 22517; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South && self.r#waterlogged == false { return 22492; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::Straight { return 22506; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft { return 22477; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#facing == Facing::South { return 22485; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 22484; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == true { return 22495; }
        if self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#waterlogged == true { return 22503; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::West { return 22498; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::InnerRight { return 22500; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#facing == Facing::North && self.r#half == Half::Top { return 22463; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#facing == Facing::South { return 22494; }
        if self.r#facing == Facing::West && self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#half == Half::Bottom { return 22510; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South && self.r#half == Half::Top { return 22482; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#half == Half::Top { return 22515; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 22471; }
        if self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#half == Half::Top { return 22481; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 22514; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 22467; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#waterlogged == false { return 22462; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom { return 22469; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#shape == Shape::InnerRight && self.r#half == Half::Top { return 22480; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::Straight { return 22455; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 22509; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East && self.r#half == Half::Top { return 22521; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#half == Half::Bottom { return 22531; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#facing == Facing::East { return 22534; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Top { return 22458; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#facing == Facing::South { return 22486; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft { return 22502; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == false { return 22522; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 22508; }
        if self.r#facing == Facing::North && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#waterlogged == false { return 22472; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == true { return 22513; }
        if self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#waterlogged == false { return 22468; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#shape == Shape::InnerRight { return 22489; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top { return 22497; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#waterlogged == false { return 22466; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::Straight { return 22476; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == false { return 22460; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::West { return 22504; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#shape == Shape::Straight { return 22505; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East && self.r#waterlogged == false { return 22518; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::North { return 22473; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::InnerRight { return 22520; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom && self.r#facing == Facing::East { return 22527; }
        if self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == false { return 22528; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::OuterLeft { return 22512; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom { return 22491; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Top { return 22499; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::InnerRight { return 22470; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 22487; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Top { return 22461; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::OuterRight { return 22474; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::East { return 22529; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 22475; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#facing == Facing::East { return 22533; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::South { return 22483; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom { return 22488; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top { return 22457; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft { return 22478; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#shape == Shape::Straight { return 22465; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom { return 22490; }
        if self.r#facing == Facing::West && self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#waterlogged == false { return 22496; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::OuterRight { return 22464; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::West { return 22511; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Top { return 22459; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::West { return 22507; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::InnerRight { return 22519; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#facing == Facing::East { return 22523; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::Straight { return 22525; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 22526; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::InnerRight { return 22530; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#facing == Facing::South { return 22493; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 22479; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 22456; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22516 {
            return Some(PolishedBlackstoneStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 22501 {
            return Some(PolishedBlackstoneStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 22524 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 22532 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 22517 {
            return Some(PolishedBlackstoneStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 22492 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 22506 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 22477 {
            return Some(PolishedBlackstoneStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 22485 {
            return Some(PolishedBlackstoneStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
            });
        }
        if state_id == 22484 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 22495 {
            return Some(PolishedBlackstoneStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 22503 {
            return Some(PolishedBlackstoneStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 22498 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 22500 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 22463 {
            return Some(PolishedBlackstoneStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 22494 {
            return Some(PolishedBlackstoneStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
            });
        }
        if state_id == 22510 {
            return Some(PolishedBlackstoneStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 22482 {
            return Some(PolishedBlackstoneStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 22515 {
            return Some(PolishedBlackstoneStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 22471 {
            return Some(PolishedBlackstoneStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 22481 {
            return Some(PolishedBlackstoneStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 22514 {
            return Some(PolishedBlackstoneStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 22467 {
            return Some(PolishedBlackstoneStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 22462 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 22469 {
            return Some(PolishedBlackstoneStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 22480 {
            return Some(PolishedBlackstoneStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 22455 {
            return Some(PolishedBlackstoneStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 22509 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 22521 {
            return Some(PolishedBlackstoneStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 22531 {
            return Some(PolishedBlackstoneStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 22534 {
            return Some(PolishedBlackstoneStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 22458 {
            return Some(PolishedBlackstoneStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 22486 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 22502 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 22522 {
            return Some(PolishedBlackstoneStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 22508 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 22472 {
            return Some(PolishedBlackstoneStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 22513 {
            return Some(PolishedBlackstoneStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 22468 {
            return Some(PolishedBlackstoneStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 22489 {
            return Some(PolishedBlackstoneStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 22497 {
            return Some(PolishedBlackstoneStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 22466 {
            return Some(PolishedBlackstoneStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 22476 {
            return Some(PolishedBlackstoneStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 22460 {
            return Some(PolishedBlackstoneStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 22504 {
            return Some(PolishedBlackstoneStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 22505 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 22518 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 22473 {
            return Some(PolishedBlackstoneStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 22520 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 22527 {
            return Some(PolishedBlackstoneStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 22528 {
            return Some(PolishedBlackstoneStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 22512 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 22491 {
            return Some(PolishedBlackstoneStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 22499 {
            return Some(PolishedBlackstoneStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 22470 {
            return Some(PolishedBlackstoneStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 22487 {
            return Some(PolishedBlackstoneStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 22461 {
            return Some(PolishedBlackstoneStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 22474 {
            return Some(PolishedBlackstoneStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 22529 {
            return Some(PolishedBlackstoneStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 22475 {
            return Some(PolishedBlackstoneStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 22533 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 22483 {
            return Some(PolishedBlackstoneStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 22488 {
            return Some(PolishedBlackstoneStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 22457 {
            return Some(PolishedBlackstoneStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 22478 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 22465 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 22490 {
            return Some(PolishedBlackstoneStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 22496 {
            return Some(PolishedBlackstoneStairs {
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 22464 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 22511 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 22459 {
            return Some(PolishedBlackstoneStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 22507 {
            return Some(PolishedBlackstoneStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 22519 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 22523 {
            return Some(PolishedBlackstoneStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 22525 {
            return Some(PolishedBlackstoneStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 22526 {
            return Some(PolishedBlackstoneStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 22530 {
            return Some(PolishedBlackstoneStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 22493 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
            });
        }
        if state_id == 22479 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 22456 {
            return Some(PolishedBlackstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        return None;
    }
}

