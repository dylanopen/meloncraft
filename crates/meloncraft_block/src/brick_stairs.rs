use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrickStairs {
    pub r#half: Half,
    pub waterlogged: bool,
    pub r#facing: Facing,
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

impl BlockState for BrickStairs {
    fn to_id(self) -> i32 {
        if block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 8518; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true { return 8539; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 8492; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 8545; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft { return 8554; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft { return 8499; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::South { return 8497; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North { return 8495; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 8502; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::South { return 8506; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true { return 8517; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 8549; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top { return 8540; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 8550; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 8524; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight { return 8541; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft { return 8489; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::South { return 8501; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight { return 8516; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight { return 8478; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight { return 8488; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft { return 8519; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::West { return 8520; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 8533; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 8535; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false { return 8482; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerLeft { return 8479; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight { return 8515; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight { return 8486; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false { return 8548; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 8556; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true { return 8523; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 8538; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterRight { return 8505; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 8490; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerRight { return 8532; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 8555; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft { return 8500; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft { return 8510; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#facing == Facing::West { return 8522; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::East { return 8542; }
        if block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom { return 8508; }
        if block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::South { return 8498; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North { return 8491; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 8509; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 8513; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Top { return 8484; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 8485; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight { return 8481; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight { return 8521; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 8544; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South { return 8512; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight { return 8525; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom { return 8507; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom { return 8514; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::North { return 8483; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerLeft { return 8530; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom { return 8487; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom { return 8496; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 8504; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false { return 8526; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 8531; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 8511; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true { return 8537; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft { return 8480; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight { return 8546; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 8551; }
        if block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 8527; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 8553; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom { return 8528; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::South && block_state.r#half == Half::Top { return 8503; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 8494; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true { return 8529; }
        if block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 8493; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft { return 8534; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterLeft { return 8543; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::West { return 8536; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight { return 8547; }
        if block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 8477; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 8552; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8518 {
            return Some(BrickStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 8539 {
            return Some(BrickStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 8492 {
            return Some(BrickStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 8545 {
            return Some(BrickStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 8554 {
            return Some(BrickStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 8499 {
            return Some(BrickStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 8497 {
            return Some(BrickStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
            });
        }
        if state_id == 8495 {
            return Some(BrickStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 8502 {
            return Some(BrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 8506 {
            return Some(BrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
            });
        }
        if state_id == 8517 {
            return Some(BrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 8549 {
            return Some(BrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 8540 {
            return Some(BrickStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 8550 {
            return Some(BrickStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 8524 {
            return Some(BrickStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 8541 {
            return Some(BrickStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 8489 {
            return Some(BrickStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 8501 {
            return Some(BrickStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
            });
        }
        if state_id == 8516 {
            return Some(BrickStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 8478 {
            return Some(BrickStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 8488 {
            return Some(BrickStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 8519 {
            return Some(BrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 8520 {
            return Some(BrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 8533 {
            return Some(BrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 8535 {
            return Some(BrickStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 8482 {
            return Some(BrickStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 8479 {
            return Some(BrickStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 8515 {
            return Some(BrickStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 8486 {
            return Some(BrickStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 8548 {
            return Some(BrickStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 8556 {
            return Some(BrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8523 {
            return Some(BrickStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 8538 {
            return Some(BrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 8505 {
            return Some(BrickStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
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
        if state_id == 8532 {
            return Some(BrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 8555 {
            return Some(BrickStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 8500 {
            return Some(BrickStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 8510 {
            return Some(BrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 8522 {
            return Some(BrickStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 8542 {
            return Some(BrickStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 8508 {
            return Some(BrickStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8498 {
            return Some(BrickStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 8491 {
            return Some(BrickStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
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
        if state_id == 8513 {
            return Some(BrickStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8484 {
            return Some(BrickStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 8485 {
            return Some(BrickStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 8481 {
            return Some(BrickStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 8521 {
            return Some(BrickStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 8544 {
            return Some(BrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 8512 {
            return Some(BrickStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 8525 {
            return Some(BrickStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 8507 {
            return Some(BrickStairs {
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8514 {
            return Some(BrickStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8483 {
            return Some(BrickStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 8530 {
            return Some(BrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 8487 {
            return Some(BrickStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8496 {
            return Some(BrickStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8504 {
            return Some(BrickStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 8526 {
            return Some(BrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 8531 {
            return Some(BrickStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 8511 {
            return Some(BrickStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8537 {
            return Some(BrickStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 8480 {
            return Some(BrickStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 8546 {
            return Some(BrickStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 8551 {
            return Some(BrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 8527 {
            return Some(BrickStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 8553 {
            return Some(BrickStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 8528 {
            return Some(BrickStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8503 {
            return Some(BrickStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 8494 {
            return Some(BrickStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 8529 {
            return Some(BrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 8493 {
            return Some(BrickStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8534 {
            return Some(BrickStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 8543 {
            return Some(BrickStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 8536 {
            return Some(BrickStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 8547 {
            return Some(BrickStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 8477 {
            return Some(BrickStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 8552 {
            return Some(BrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

