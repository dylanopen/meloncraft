use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MudBrickStairs {
    pub r#facing: Facing,
    pub r#half: Half,
    pub r#shape: Shape,
    pub waterlogged: bool,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Straight,
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight,
}

impl BlockState for MudBrickStairs {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#half == Half::Top { return 8678; }
        if self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#facing == Facing::North { return 8637; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 8663; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#facing == Facing::East && self.r#waterlogged == false { return 8706; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == true { return 8681; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::Straight { return 8638; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 8700; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 8703; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 8689; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::Straight { return 8647; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == true { return 8691; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top { return 8644; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 8653; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft { return 8714; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#facing == Facing::North { return 8639; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == false { return 8662; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::North && self.r#waterlogged == true { return 8649; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::East { return 8708; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East && self.r#waterlogged == true { return 8713; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::North { return 8650; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::Straight { return 8658; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::North { return 8656; }
        if self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#facing == Facing::West { return 8677; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 8682; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 8696; }
        if self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 8692; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 8654; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#half == Half::Top { return 8646; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::North { return 8652; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft { return 8670; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#facing == Facing::West { return 8686; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South { return 8673; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#half == Half::Top { return 8685; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == true { return 8645; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft { return 8694; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#facing == Facing::East && self.r#waterlogged == false { return 8702; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 8641; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 8661; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#facing == Facing::West { return 8679; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#waterlogged == false { return 8668; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == false { return 8642; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top { return 8664; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 8693; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == true { return 8705; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::South { return 8672; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 8680; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight { return 8651; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#waterlogged == true { return 8643; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Bottom { return 8712; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 8704; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#shape == Shape::InnerRight { return 8671; }
        if self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#facing == Facing::East { return 8697; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == true { return 8657; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::East { return 8699; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 8655; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::InnerRight && self.r#half == Half::Top { return 8701; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#waterlogged == false { return 8688; }
        if self.r#facing == Facing::South && self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#waterlogged == true { return 8675; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft { return 8640; }
        if self.r#facing == Facing::South && self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#waterlogged == false { return 8666; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#facing == Facing::South { return 8676; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::South { return 8674; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Bottom { return 8711; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == true { return 8659; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#facing == Facing::West { return 8684; }
        if self.r#facing == Facing::East && self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#half == Half::Bottom { return 8715; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::East { return 8698; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::South { return 8667; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 8690; }
        if self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom && self.r#waterlogged == true { return 8709; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Top { return 8683; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Bottom { return 8687; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::Straight { return 8707; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft { return 8660; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#waterlogged == false { return 8710; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#facing == Facing::North { return 8648; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#facing == Facing::East { return 8716; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#facing == Facing::South && self.r#waterlogged == true { return 8665; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 8669; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#facing == Facing::West { return 8695; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8678 {
            return Some(MudBrickStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 8637 {
            return Some(MudBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 8663 {
            return Some(MudBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 8706 {
            return Some(MudBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 8681 {
            return Some(MudBrickStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 8638 {
            return Some(MudBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 8700 {
            return Some(MudBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 8703 {
            return Some(MudBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 8689 {
            return Some(MudBrickStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 8647 {
            return Some(MudBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 8691 {
            return Some(MudBrickStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 8644 {
            return Some(MudBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 8653 {
            return Some(MudBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 8714 {
            return Some(MudBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 8639 {
            return Some(MudBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 8662 {
            return Some(MudBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 8649 {
            return Some(MudBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 8708 {
            return Some(MudBrickStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 8713 {
            return Some(MudBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 8650 {
            return Some(MudBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 8658 {
            return Some(MudBrickStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 8656 {
            return Some(MudBrickStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 8677 {
            return Some(MudBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 8682 {
            return Some(MudBrickStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 8696 {
            return Some(MudBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 8692 {
            return Some(MudBrickStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8654 {
            return Some(MudBrickStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8646 {
            return Some(MudBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 8652 {
            return Some(MudBrickStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 8670 {
            return Some(MudBrickStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 8686 {
            return Some(MudBrickStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 8673 {
            return Some(MudBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 8685 {
            return Some(MudBrickStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 8645 {
            return Some(MudBrickStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 8694 {
            return Some(MudBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 8702 {
            return Some(MudBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 8641 {
            return Some(MudBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 8661 {
            return Some(MudBrickStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 8679 {
            return Some(MudBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 8668 {
            return Some(MudBrickStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 8642 {
            return Some(MudBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 8664 {
            return Some(MudBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 8693 {
            return Some(MudBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8705 {
            return Some(MudBrickStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 8672 {
            return Some(MudBrickStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 8680 {
            return Some(MudBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 8651 {
            return Some(MudBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 8643 {
            return Some(MudBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 8712 {
            return Some(MudBrickStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8704 {
            return Some(MudBrickStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 8671 {
            return Some(MudBrickStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 8697 {
            return Some(MudBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 8657 {
            return Some(MudBrickStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 8699 {
            return Some(MudBrickStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 8655 {
            return Some(MudBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 8701 {
            return Some(MudBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 8688 {
            return Some(MudBrickStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 8675 {
            return Some(MudBrickStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 8640 {
            return Some(MudBrickStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 8666 {
            return Some(MudBrickStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 8676 {
            return Some(MudBrickStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
            });
        }
        if state_id == 8674 {
            return Some(MudBrickStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 8711 {
            return Some(MudBrickStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8659 {
            return Some(MudBrickStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 8684 {
            return Some(MudBrickStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 8715 {
            return Some(MudBrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8698 {
            return Some(MudBrickStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 8667 {
            return Some(MudBrickStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 8690 {
            return Some(MudBrickStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8709 {
            return Some(MudBrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 8683 {
            return Some(MudBrickStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 8687 {
            return Some(MudBrickStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8707 {
            return Some(MudBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 8660 {
            return Some(MudBrickStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 8710 {
            return Some(MudBrickStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 8648 {
            return Some(MudBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 8716 {
            return Some(MudBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 8665 {
            return Some(MudBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 8669 {
            return Some(MudBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 8695 {
            return Some(MudBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}

