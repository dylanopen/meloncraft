use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpruceStairs {
    pub waterlogged: bool,
    pub r#facing: Facing,
    pub r#shape: Shape,
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

impl BlockState for SpruceStairs {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::OuterRight { return 9545; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Bottom { return 9558; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Bottom { return 9577; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 9550; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::West { return 9583; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == false { return 9562; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == true { return 9557; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == false { return 9584; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == false { return 9590; }
        if self.r#waterlogged == true && self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#facing == Facing::North { return 9537; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::East { return 9604; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::South { return 9547; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == false { return 9546; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::South { return 9565; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top { return 9574; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 9591; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 9593; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#facing == Facing::East { return 9606; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == false { return 9534; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 9587; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#facing == Facing::South { return 9555; }
        if self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 9582; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#shape == Shape::InnerRight { return 9542; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft { return 9563; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 9539; }
        if self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#half == Half::Bottom { return 9540; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#shape == Shape::Straight { return 9527; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#facing == Facing::North { return 9536; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::West { return 9586; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#facing == Facing::West && self.r#waterlogged == true { return 9575; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East { return 9600; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#facing == Facing::West && self.r#waterlogged == true { return 9585; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::West { return 9580; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::InnerRight { return 9602; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::South { return 9566; }
        if self.r#facing == Facing::West && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#waterlogged == true { return 9581; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#facing == Facing::East && self.r#half == Half::Top { return 9596; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#facing == Facing::North { return 9538; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 9605; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft { return 9589; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#half == Half::Top { return 9572; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft { return 9553; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 9598; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#facing == Facing::North { return 9532; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#facing == Facing::South && self.r#half == Half::Top { return 9556; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top { return 9529; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom { return 9560; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#shape == Shape::InnerRight { return 9571; }
        if self.r#facing == Facing::East && self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#half == Half::Top { return 9595; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#half == Half::Bottom { return 9597; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Top { return 9528; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 9578; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == true { return 9603; }
        if self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#waterlogged == false { return 9554; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 9543; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#facing == Facing::East { return 9592; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#waterlogged == true { return 9561; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft { return 9570; }
        if self.r#facing == Facing::East && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#waterlogged == true { return 9601; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Top { return 9569; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom && self.r#facing == Facing::East { return 9599; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 9573; }
        if self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#half == Half::Top { return 9588; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::InnerRight { return 9541; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 9594; }
        if self.r#facing == Facing::South && self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#half == Half::Top { return 9551; }
        if self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::South { return 9549; }
        if self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#half == Half::Top { return 9530; }
        if self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#waterlogged == false { return 9576; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 9579; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::Straight { return 9568; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#half == Half::Top { return 9548; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::North { return 9533; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#shape == Shape::InnerRight { return 9552; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 9544; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#shape == Shape::InnerRight { return 9531; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#facing == Facing::South { return 9564; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Bottom { return 9559; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Top { return 9567; }
        if self.r#facing == Facing::North && self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#half == Half::Top { return 9535; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9545 {
            return Some(SpruceStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 9558 {
            return Some(SpruceStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9577 {
            return Some(SpruceStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9550 {
            return Some(SpruceStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 9583 {
            return Some(SpruceStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 9562 {
            return Some(SpruceStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 9557 {
            return Some(SpruceStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 9584 {
            return Some(SpruceStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 9590 {
            return Some(SpruceStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 9537 {
            return Some(SpruceStairs {
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 9604 {
            return Some(SpruceStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 9547 {
            return Some(SpruceStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 9546 {
            return Some(SpruceStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 9565 {
            return Some(SpruceStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 9574 {
            return Some(SpruceStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 9591 {
            return Some(SpruceStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 9593 {
            return Some(SpruceStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 9606 {
            return Some(SpruceStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 9534 {
            return Some(SpruceStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 9587 {
            return Some(SpruceStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 9555 {
            return Some(SpruceStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 9582 {
            return Some(SpruceStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9542 {
            return Some(SpruceStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 9563 {
            return Some(SpruceStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 9539 {
            return Some(SpruceStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 9540 {
            return Some(SpruceStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9527 {
            return Some(SpruceStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 9536 {
            return Some(SpruceStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
            });
        }
        if state_id == 9586 {
            return Some(SpruceStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 9575 {
            return Some(SpruceStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 9600 {
            return Some(SpruceStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 9585 {
            return Some(SpruceStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 9580 {
            return Some(SpruceStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 9602 {
            return Some(SpruceStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 9566 {
            return Some(SpruceStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 9581 {
            return Some(SpruceStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 9596 {
            return Some(SpruceStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 9538 {
            return Some(SpruceStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
            });
        }
        if state_id == 9605 {
            return Some(SpruceStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 9589 {
            return Some(SpruceStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 9572 {
            return Some(SpruceStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 9553 {
            return Some(SpruceStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 9598 {
            return Some(SpruceStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 9532 {
            return Some(SpruceStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 9556 {
            return Some(SpruceStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 9529 {
            return Some(SpruceStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 9560 {
            return Some(SpruceStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9571 {
            return Some(SpruceStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 9595 {
            return Some(SpruceStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 9597 {
            return Some(SpruceStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9528 {
            return Some(SpruceStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 9578 {
            return Some(SpruceStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 9603 {
            return Some(SpruceStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 9554 {
            return Some(SpruceStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 9543 {
            return Some(SpruceStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 9592 {
            return Some(SpruceStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 9561 {
            return Some(SpruceStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 9570 {
            return Some(SpruceStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 9601 {
            return Some(SpruceStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 9569 {
            return Some(SpruceStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 9599 {
            return Some(SpruceStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 9573 {
            return Some(SpruceStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 9588 {
            return Some(SpruceStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 9541 {
            return Some(SpruceStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 9594 {
            return Some(SpruceStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 9551 {
            return Some(SpruceStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 9549 {
            return Some(SpruceStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 9530 {
            return Some(SpruceStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 9576 {
            return Some(SpruceStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 9579 {
            return Some(SpruceStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 9568 {
            return Some(SpruceStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 9548 {
            return Some(SpruceStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 9533 {
            return Some(SpruceStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 9552 {
            return Some(SpruceStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 9544 {
            return Some(SpruceStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9531 {
            return Some(SpruceStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 9564 {
            return Some(SpruceStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 9559 {
            return Some(SpruceStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9567 {
            return Some(SpruceStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 9535 {
            return Some(SpruceStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        return None;
    }
}

