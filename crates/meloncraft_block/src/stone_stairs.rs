use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoneStairs {
    pub r#facing: Facing,
    pub r#half: Half,
    pub waterlogged: bool,
    pub r#shape: Shape,
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

impl BlockState for StoneStairs {
    fn to_id(&self) -> i32 {
        if self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 15624; }
        if self.r#facing == Facing::South && self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#waterlogged == false { return 15595; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == false { return 15583; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::North { return 15577; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top { return 15641; }
        if self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom && self.r#waterlogged == true { return 15586; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Bottom { return 15587; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::InnerRight { return 15598; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::OuterRight { return 15592; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 15631; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#facing == Facing::South { return 15608; }
        if self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::West { return 15616; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 15604; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom { return 15647; }
        if self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#facing == Facing::West { return 15615; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#shape == Shape::OuterRight { return 15612; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#facing == Facing::North { return 15584; }
        if self.r#facing == Facing::North && self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#waterlogged == false { return 15593; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft { return 15576; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 15589; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#facing == Facing::South { return 15596; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#facing == Facing::West { return 15619; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::InnerRight { return 15579; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 15638; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#waterlogged == true { return 15610; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 15642; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::South { return 15597; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 15635; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 15581; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South && self.r#waterlogged == false { return 15611; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 15628; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#waterlogged == true { return 15632; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#facing == Facing::East { return 15646; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft { return 15626; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == false { return 15645; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::OuterRight { return 15652; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::InnerRight { return 15599; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#half == Half::Bottom { return 15585; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#waterlogged == false { return 15653; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#half == Half::Bottom { return 15644; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::OuterRight { return 15643; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#shape == Shape::InnerRight { return 15609; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom { return 15607; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 15580; }
        if self.r#facing == Facing::West && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#waterlogged == true { return 15620; }
        if self.r#facing == Facing::North && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#half == Half::Bottom { return 15591; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#facing == Facing::South && self.r#waterlogged == true { return 15602; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::Straight && self.r#half == Half::Top { return 15594; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom { return 15629; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#waterlogged == true { return 15590; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#facing == Facing::South { return 15605; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#half == Half::Bottom { return 15625; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Bottom { return 15588; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft { return 15601; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#facing == Facing::West { return 15618; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft { return 15600; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::OuterLeft { return 15621; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#facing == Facing::West { return 15622; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 15630; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#half == Half::Top { return 15603; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#half == Half::Top { return 15636; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft { return 15637; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#shape == Shape::InnerRight { return 15578; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Bottom { return 15606; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 15617; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::Straight { return 15634; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 15640; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::InnerRight { return 15649; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#facing == Facing::West { return 15633; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#half == Half::Top { return 15582; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::OuterRight { return 15623; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#half == Half::Top { return 15639; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#shape == Shape::Straight { return 15574; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#facing == Facing::South && self.r#waterlogged == false { return 15613; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 15614; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 15627; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom { return 15650; }
        if self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#waterlogged == false { return 15651; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#facing == Facing::East && self.r#waterlogged == true { return 15648; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Top { return 15575; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15624 {
            return Some(StoneStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15595 {
            return Some(StoneStairs {
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 15583 {
            return Some(StoneStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 15577 {
            return Some(StoneStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 15641 {
            return Some(StoneStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 15586 {
            return Some(StoneStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 15587 {
            return Some(StoneStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15598 {
            return Some(StoneStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15592 {
            return Some(StoneStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15631 {
            return Some(StoneStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15608 {
            return Some(StoneStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 15616 {
            return Some(StoneStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 15604 {
            return Some(StoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 15647 {
            return Some(StoneStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15615 {
            return Some(StoneStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 15612 {
            return Some(StoneStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15584 {
            return Some(StoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 15593 {
            return Some(StoneStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 15576 {
            return Some(StoneStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15589 {
            return Some(StoneStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15596 {
            return Some(StoneStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 15619 {
            return Some(StoneStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 15579 {
            return Some(StoneStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15638 {
            return Some(StoneStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15610 {
            return Some(StoneStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 15642 {
            return Some(StoneStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15597 {
            return Some(StoneStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 15635 {
            return Some(StoneStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15581 {
            return Some(StoneStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15611 {
            return Some(StoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 15628 {
            return Some(StoneStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15632 {
            return Some(StoneStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 15646 {
            return Some(StoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 15626 {
            return Some(StoneStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15645 {
            return Some(StoneStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 15652 {
            return Some(StoneStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15599 {
            return Some(StoneStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15585 {
            return Some(StoneStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15653 {
            return Some(StoneStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 15644 {
            return Some(StoneStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15643 {
            return Some(StoneStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15609 {
            return Some(StoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15607 {
            return Some(StoneStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15580 {
            return Some(StoneStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 15620 {
            return Some(StoneStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 15591 {
            return Some(StoneStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15602 {
            return Some(StoneStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 15594 {
            return Some(StoneStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 15629 {
            return Some(StoneStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15590 {
            return Some(StoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 15605 {
            return Some(StoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
            });
        }
        if state_id == 15625 {
            return Some(StoneStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15588 {
            return Some(StoneStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15601 {
            return Some(StoneStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15618 {
            return Some(StoneStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 15600 {
            return Some(StoneStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15621 {
            return Some(StoneStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15622 {
            return Some(StoneStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 15630 {
            return Some(StoneStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15603 {
            return Some(StoneStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 15636 {
            return Some(StoneStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 15637 {
            return Some(StoneStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15578 {
            return Some(StoneStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15606 {
            return Some(StoneStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15617 {
            return Some(StoneStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 15634 {
            return Some(StoneStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15640 {
            return Some(StoneStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 15649 {
            return Some(StoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15633 {
            return Some(StoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 15582 {
            return Some(StoneStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 15623 {
            return Some(StoneStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15639 {
            return Some(StoneStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 15574 {
            return Some(StoneStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15613 {
            return Some(StoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 15614 {
            return Some(StoneStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 15627 {
            return Some(StoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 15650 {
            return Some(StoneStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15651 {
            return Some(StoneStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 15648 {
            return Some(StoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 15575 {
            return Some(StoneStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        return None;
    }
}

