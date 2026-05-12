use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoneBrickStairs {
    pub r#facing: Facing,
    pub waterlogged: bool,
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

impl BlockState for StoneBrickStairs {
    fn to_id(&self) -> i32 {
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft { return 8609; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 8578; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#facing == Facing::North { return 8575; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 8583; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#shape == Shape::OuterRight { return 8596; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::InnerRight { return 8612; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::South { return 8590; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == true { return 8561; }
        if self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#half == Half::Bottom { return 8629; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 8567; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom { return 8630; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#facing == Facing::East { return 8619; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::InnerRight { return 8631; }
        if self.r#facing == Facing::East && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#waterlogged == false { return 8632; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft { return 8634; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft { return 8610; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom { return 8633; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#facing == Facing::East && self.r#half == Half::Top { return 8625; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#facing == Facing::North && self.r#waterlogged == false { return 8572; }
        if self.r#facing == Facing::South && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#waterlogged == false { return 8592; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::West { return 8607; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 8560; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#facing == Facing::North { return 8571; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == false { return 8582; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#half == Half::Top { return 8617; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::North { return 8574; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::InnerRight && self.r#half == Half::Top { return 8622; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 8563; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::InnerRight && self.r#waterlogged == false { return 8562; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 8564; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 8621; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft { return 8603; }
        if self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#waterlogged == false { return 8628; }
        if self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom && self.r#waterlogged == true { return 8589; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::West { return 8613; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 8624; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight { return 8611; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Top { return 8604; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#facing == Facing::West { return 8602; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft { return 8579; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 8600; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::Straight { return 8598; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#half == Half::Top { return 8558; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 8626; }
        if self.r#facing == Facing::South && self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#waterlogged == false { return 8586; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 8593; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 8635; }
        if self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom && self.r#facing == Facing::North { return 8570; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#shape == Shape::Straight && self.r#waterlogged == false { return 8588; }
        if self.r#waterlogged == true && self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#facing == Facing::North { return 8557; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#waterlogged == false { return 8568; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft { return 8620; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#waterlogged == true { return 8573; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 8559; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom { return 8594; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::West { return 8599; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::OuterLeft { return 8614; }
        if self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#half == Half::Bottom { return 8616; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::North { return 8569; }
        if self.r#facing == Facing::North && self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#waterlogged == true { return 8565; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 8587; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == false { return 8608; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#facing == Facing::South { return 8581; }
        if self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#waterlogged == false { return 8584; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 8595; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::OuterRight { return 8576; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::InnerRight { return 8591; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::OuterRight { return 8566; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == true { return 8585; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#half == Half::Top { return 8618; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft { return 8580; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft { return 8623; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#facing == Facing::West { return 8615; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::Straight { return 8627; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Top { return 8597; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#half == Half::Top { return 8605; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#half == Half::Top { return 8606; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom { return 8636; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 8601; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#half == Half::Top { return 8577; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8609 {
            return Some(StoneBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 8578 {
            return Some(StoneBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 8575 {
            return Some(StoneBrickStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
            });
        }
        if state_id == 8583 {
            return Some(StoneBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 8596 {
            return Some(StoneBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 8612 {
            return Some(StoneBrickStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 8590 {
            return Some(StoneBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 8561 {
            return Some(StoneBrickStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 8629 {
            return Some(StoneBrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8567 {
            return Some(StoneBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 8630 {
            return Some(StoneBrickStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8619 {
            return Some(StoneBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 8631 {
            return Some(StoneBrickStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 8632 {
            return Some(StoneBrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 8634 {
            return Some(StoneBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 8610 {
            return Some(StoneBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 8633 {
            return Some(StoneBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8625 {
            return Some(StoneBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 8572 {
            return Some(StoneBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 8592 {
            return Some(StoneBrickStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 8607 {
            return Some(StoneBrickStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 8560 {
            return Some(StoneBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 8571 {
            return Some(StoneBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 8582 {
            return Some(StoneBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 8617 {
            return Some(StoneBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 8574 {
            return Some(StoneBrickStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 8622 {
            return Some(StoneBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 8563 {
            return Some(StoneBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 8562 {
            return Some(StoneBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 8564 {
            return Some(StoneBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 8621 {
            return Some(StoneBrickStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 8603 {
            return Some(StoneBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 8628 {
            return Some(StoneBrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 8589 {
            return Some(StoneBrickStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 8613 {
            return Some(StoneBrickStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 8624 {
            return Some(StoneBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 8611 {
            return Some(StoneBrickStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 8604 {
            return Some(StoneBrickStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 8602 {
            return Some(StoneBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 8579 {
            return Some(StoneBrickStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 8600 {
            return Some(StoneBrickStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 8598 {
            return Some(StoneBrickStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 8558 {
            return Some(StoneBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 8626 {
            return Some(StoneBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 8586 {
            return Some(StoneBrickStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 8593 {
            return Some(StoneBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 8635 {
            return Some(StoneBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 8570 {
            return Some(StoneBrickStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 8588 {
            return Some(StoneBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 8557 {
            return Some(StoneBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 8568 {
            return Some(StoneBrickStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 8620 {
            return Some(StoneBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 8573 {
            return Some(StoneBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 8559 {
            return Some(StoneBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 8594 {
            return Some(StoneBrickStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8599 {
            return Some(StoneBrickStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 8614 {
            return Some(StoneBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 8616 {
            return Some(StoneBrickStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8569 {
            return Some(StoneBrickStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 8565 {
            return Some(StoneBrickStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 8587 {
            return Some(StoneBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 8608 {
            return Some(StoneBrickStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 8581 {
            return Some(StoneBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 8584 {
            return Some(StoneBrickStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 8595 {
            return Some(StoneBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 8576 {
            return Some(StoneBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 8591 {
            return Some(StoneBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 8566 {
            return Some(StoneBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 8585 {
            return Some(StoneBrickStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 8618 {
            return Some(StoneBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 8580 {
            return Some(StoneBrickStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 8623 {
            return Some(StoneBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 8615 {
            return Some(StoneBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 8627 {
            return Some(StoneBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 8597 {
            return Some(StoneBrickStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 8605 {
            return Some(StoneBrickStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 8606 {
            return Some(StoneBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 8636 {
            return Some(StoneBrickStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 8601 {
            return Some(StoneBrickStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 8577 {
            return Some(StoneBrickStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        return None;
    }
}

