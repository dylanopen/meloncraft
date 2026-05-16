use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrismarineStairs {
    pub r#facing: Facing,
    pub r#shape: Shape,
    pub waterlogged: bool,
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
pub enum Shape {
    Straight,
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Top,
    Bottom,
}

impl BlockState for PrismarineStairs {
    fn to_id(&self) -> i32 {
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::East { return 12496; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom { return 12465; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::Straight { return 12473; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::InnerRight { return 12437; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top { return 12479; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#facing == Facing::East && self.r#half == Half::Top { return 12500; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#waterlogged == false { return 12493; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Top { return 12478; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::OuterRight { return 12460; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::Straight { return 12502; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 12511; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#facing == Facing::West { return 12483; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#shape == Shape::OuterLeft { return 12448; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::West { return 12489; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom { return 12504; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::West { return 12475; }
        if self.r#facing == Facing::South && self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#half == Half::Bottom { return 12463; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 12466; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::East { return 12492; }
        if self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#half == Half::Top { return 12458; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East { return 12508; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::OuterRight { return 12461; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 12439; }
        if self.r#facing == Facing::North && self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#waterlogged == false { return 12433; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom { return 12450; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#half == Half::Bottom { return 12468; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 12509; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft { return 12484; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 12474; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::InnerRight { return 12487; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#facing == Facing::West { return 12477; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 12485; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Top { return 12472; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::South { return 12454; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == true { return 12444; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 12470; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#shape == Shape::OuterLeft { return 12488; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft { return 12495; }
        if self.r#facing == Facing::North && self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#waterlogged == true { return 12432; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#facing == Facing::North && self.r#waterlogged == true { return 12446; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::South { return 12467; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft { return 12494; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top { return 12498; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 12510; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 12480; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#facing == Facing::East { return 12506; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 12469; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == false { return 12459; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 12505; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::Straight { return 12482; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Top { return 12476; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 12441; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::Straight { return 12453; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#shape == Shape::OuterRight { return 12490; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::North { return 12451; }
        if self.r#facing == Facing::South && self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#half == Half::Bottom { return 12471; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 12499; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#facing == Facing::East { return 12503; }
        if self.r#facing == Facing::North && self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#waterlogged == true { return 12436; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::West { return 12491; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft { return 12434; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#facing == Facing::North && self.r#half == Half::Top { return 12440; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::InnerRight { return 12497; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 12501; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#facing == Facing::West { return 12481; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::InnerRight { return 12447; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::Straight { return 12462; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 12449; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::South { return 12464; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 12456; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == false { return 12457; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::North { return 12435; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North { return 12438; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#half == Half::Top { return 12452; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Bottom { return 12486; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft { return 12445; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#facing == Facing::East && self.r#waterlogged == false { return 12507; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft { return 12455; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#shape == Shape::Straight && self.r#half == Half::Bottom { return 12442; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Bottom { return 12443; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12496 {
            return Some(PrismarineStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 12465 {
            return Some(PrismarineStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12473 {
            return Some(PrismarineStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12437 {
            return Some(PrismarineStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 12479 {
            return Some(PrismarineStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 12500 {
            return Some(PrismarineStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 12493 {
            return Some(PrismarineStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 12478 {
            return Some(PrismarineStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 12460 {
            return Some(PrismarineStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 12502 {
            return Some(PrismarineStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12511 {
            return Some(PrismarineStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 12483 {
            return Some(PrismarineStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
            });
        }
        if state_id == 12448 {
            return Some(PrismarineStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 12489 {
            return Some(PrismarineStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 12504 {
            return Some(PrismarineStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12475 {
            return Some(PrismarineStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 12463 {
            return Some(PrismarineStairs {
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12466 {
            return Some(PrismarineStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 12492 {
            return Some(PrismarineStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 12458 {
            return Some(PrismarineStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 12508 {
            return Some(PrismarineStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 12461 {
            return Some(PrismarineStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 12439 {
            return Some(PrismarineStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 12433 {
            return Some(PrismarineStairs {
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 12450 {
            return Some(PrismarineStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12468 {
            return Some(PrismarineStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12509 {
            return Some(PrismarineStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 12484 {
            return Some(PrismarineStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12474 {
            return Some(PrismarineStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 12487 {
            return Some(PrismarineStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 12477 {
            return Some(PrismarineStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 12485 {
            return Some(PrismarineStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12472 {
            return Some(PrismarineStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 12454 {
            return Some(PrismarineStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 12444 {
            return Some(PrismarineStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 12470 {
            return Some(PrismarineStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 12488 {
            return Some(PrismarineStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 12495 {
            return Some(PrismarineStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12432 {
            return Some(PrismarineStairs {
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 12446 {
            return Some(PrismarineStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 12467 {
            return Some(PrismarineStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 12494 {
            return Some(PrismarineStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12498 {
            return Some(PrismarineStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 12510 {
            return Some(PrismarineStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 12480 {
            return Some(PrismarineStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 12506 {
            return Some(PrismarineStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 12469 {
            return Some(PrismarineStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 12459 {
            return Some(PrismarineStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 12505 {
            return Some(PrismarineStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12482 {
            return Some(PrismarineStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12476 {
            return Some(PrismarineStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 12441 {
            return Some(PrismarineStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 12453 {
            return Some(PrismarineStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12490 {
            return Some(PrismarineStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 12451 {
            return Some(PrismarineStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 12471 {
            return Some(PrismarineStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12499 {
            return Some(PrismarineStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 12503 {
            return Some(PrismarineStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 12436 {
            return Some(PrismarineStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 12491 {
            return Some(PrismarineStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 12434 {
            return Some(PrismarineStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12440 {
            return Some(PrismarineStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 12497 {
            return Some(PrismarineStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 12501 {
            return Some(PrismarineStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 12481 {
            return Some(PrismarineStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 12447 {
            return Some(PrismarineStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 12462 {
            return Some(PrismarineStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12449 {
            return Some(PrismarineStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12464 {
            return Some(PrismarineStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 12456 {
            return Some(PrismarineStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 12457 {
            return Some(PrismarineStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 12435 {
            return Some(PrismarineStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 12438 {
            return Some(PrismarineStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 12452 {
            return Some(PrismarineStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 12486 {
            return Some(PrismarineStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12445 {
            return Some(PrismarineStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12507 {
            return Some(PrismarineStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 12455 {
            return Some(PrismarineStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12442 {
            return Some(PrismarineStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12443 {
            return Some(PrismarineStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        return None;
    }
}

