use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrickStairs {
    pub r#half: Half,
    pub r#shape: Shape,
    pub r#facing: Facing,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Top,
    Bottom,
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

impl BlockState for BrickStairs {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight { return 8552; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Top { return 8505; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#facing == Facing::West && self.r#waterlogged == false { return 8528; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 8544; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 8478; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#shape == Shape::Straight && self.r#half == Half::Top { return 8517; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 8495; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 8480; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Bottom { return 8512; }
        if self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom && self.r#waterlogged == true { return 8509; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#facing == Facing::South { return 8511; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 8479; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#shape == Shape::Straight { return 8477; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight { return 8531; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == true { return 8541; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 8493; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Top { return 8485; }
        if self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == true { return 8499; }
        if self.r#facing == Facing::South && self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#waterlogged == false { return 8502; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East { return 8550; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft { return 8514; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#facing == Facing::South && self.r#waterlogged == true { return 8501; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South && self.r#waterlogged == false { return 8504; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 8535; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::OuterRight { return 8536; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::North { return 8484; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Top { return 8486; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South && self.r#waterlogged == true { return 8513; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::Straight { return 8537; }
        if self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#half == Half::Bottom { return 8530; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East { return 8554; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::InnerRight { return 8491; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == false { return 8490; }
        if self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#facing == Facing::East && self.r#half == Half::Top { return 8538; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::South { return 8497; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#facing == Facing::South { return 8516; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#facing == Facing::West { return 8518; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#facing == Facing::West && self.r#waterlogged == true { return 8521; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Bottom { return 8556; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 8547; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::South { return 8500; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#waterlogged == true { return 8533; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::East { return 8545; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Top { return 8519; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom { return 8529; }
        if self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 8492; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft { return 8503; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#facing == Facing::West && self.r#waterlogged == true { return 8527; }
        if self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#facing == Facing::South && self.r#half == Half::Top { return 8498; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft { return 8524; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#waterlogged == false { return 8494; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::InnerRight && self.r#half == Half::Top { return 8522; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::Straight { return 8548; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#facing == Facing::North && self.r#waterlogged == false { return 8496; }
        if self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#waterlogged == true { return 8525; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#shape == Shape::OuterLeft { return 8483; }
        if self.r#facing == Facing::East && self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#waterlogged == false { return 8542; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 8523; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 8534; }
        if self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::East { return 8539; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 8526; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom && self.r#facing == Facing::East { return 8549; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East && self.r#waterlogged == true { return 8553; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight { return 8555; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::South { return 8510; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#waterlogged == false { return 8546; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#facing == Facing::West { return 8532; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#facing == Facing::North { return 8481; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom { return 8515; }
        if self.r#facing == Facing::North && self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#waterlogged == false { return 8482; }
        if self.r#facing == Facing::North && self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#waterlogged == true { return 8487; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 8507; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 8489; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#waterlogged == false { return 8508; }
        if self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#waterlogged == false { return 8540; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 8488; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Top { return 8520; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 8551; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East { return 8543; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#facing == Facing::South { return 8506; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8552 {
            return Some(BrickStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 8505 {
            return Some(BrickStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 8528 {
            return Some(BrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 8544 {
            return Some(BrickStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 8478 {
            return Some(BrickStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 8517 {
            return Some(BrickStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 8495 {
            return Some(BrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 8480 {
            return Some(BrickStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 8512 {
            return Some(BrickStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8509 {
            return Some(BrickStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 8511 {
            return Some(BrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 8479 {
            return Some(BrickStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 8477 {
            return Some(BrickStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 8531 {
            return Some(BrickStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 8541 {
            return Some(BrickStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 8493 {
            return Some(BrickStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 8485 {
            return Some(BrickStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 8499 {
            return Some(BrickStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 8502 {
            return Some(BrickStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 8550 {
            return Some(BrickStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 8514 {
            return Some(BrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 8501 {
            return Some(BrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 8504 {
            return Some(BrickStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 8535 {
            return Some(BrickStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8536 {
            return Some(BrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 8484 {
            return Some(BrickStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 8486 {
            return Some(BrickStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 8513 {
            return Some(BrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 8537 {
            return Some(BrickStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 8530 {
            return Some(BrickStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8554 {
            return Some(BrickStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 8491 {
            return Some(BrickStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 8490 {
            return Some(BrickStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 8538 {
            return Some(BrickStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 8497 {
            return Some(BrickStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 8516 {
            return Some(BrickStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
            });
        }
        if state_id == 8518 {
            return Some(BrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
            });
        }
        if state_id == 8521 {
            return Some(BrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 8556 {
            return Some(BrickStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8547 {
            return Some(BrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 8500 {
            return Some(BrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 8533 {
            return Some(BrickStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 8545 {
            return Some(BrickStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 8519 {
            return Some(BrickStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 8529 {
            return Some(BrickStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8492 {
            return Some(BrickStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8503 {
            return Some(BrickStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 8527 {
            return Some(BrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 8498 {
            return Some(BrickStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 8524 {
            return Some(BrickStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 8494 {
            return Some(BrickStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 8522 {
            return Some(BrickStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 8548 {
            return Some(BrickStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 8496 {
            return Some(BrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 8525 {
            return Some(BrickStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 8483 {
            return Some(BrickStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 8542 {
            return Some(BrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 8523 {
            return Some(BrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 8534 {
            return Some(BrickStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 8539 {
            return Some(BrickStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 8526 {
            return Some(BrickStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 8549 {
            return Some(BrickStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 8553 {
            return Some(BrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 8555 {
            return Some(BrickStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 8510 {
            return Some(BrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 8546 {
            return Some(BrickStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 8532 {
            return Some(BrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 8481 {
            return Some(BrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 8515 {
            return Some(BrickStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8482 {
            return Some(BrickStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 8487 {
            return Some(BrickStairs {
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 8507 {
            return Some(BrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 8489 {
            return Some(BrickStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 8508 {
            return Some(BrickStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 8540 {
            return Some(BrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 8488 {
            return Some(BrickStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8520 {
            return Some(BrickStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 8551 {
            return Some(BrickStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 8543 {
            return Some(BrickStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 8506 {
            return Some(BrickStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}

