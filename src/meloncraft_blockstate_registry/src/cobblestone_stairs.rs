use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CobblestoneStairs {
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

impl BlockState for CobblestoneStairs {
    fn to_id(&self) -> i32 {
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == false { return 5553; }
        if self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#facing == Facing::West { return 5591; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 5610; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 5588; }
        if self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#waterlogged == false { return 5623; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == true { return 5570; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::Straight { return 5576; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::West { return 5592; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft { return 5568; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 5564; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#facing == Facing::North { return 5565; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::Straight { return 5577; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::Straight { return 5586; }
        if self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#half == Half::Bottom { return 5599; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 5604; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#facing == Facing::South && self.r#waterlogged == false { return 5585; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#half == Half::Bottom { return 5597; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#shape == Shape::Straight { return 5606; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 5593; }
        if self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom && self.r#waterlogged == true { return 5578; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft { return 5579; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#facing == Facing::West { return 5590; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#facing == Facing::West { return 5594; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::Straight { return 5617; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#facing == Facing::East && self.r#waterlogged == true { return 5616; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 5619; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::OuterRight { return 5625; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::OuterRight { return 5605; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::Straight { return 5547; }
        if self.r#waterlogged == true && self.r#shape == Shape::Straight && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 5596; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom { return 5600; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 5611; }
        if self.r#facing == Facing::North && self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#half == Half::Bottom { return 5556; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 5624; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 5559; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East { return 5618; }
        if self.r#facing == Facing::West && self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#half == Half::Top { return 5587; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft { return 5558; }
        if self.r#facing == Facing::South && self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#half == Half::Top { return 5567; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::InnerRight { return 5601; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 5562; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom { return 5603; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::East { return 5607; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#facing == Facing::North { return 5555; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top { return 5612; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#facing == Facing::East { return 5614; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#shape == Shape::Straight { return 5557; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::East { return 5615; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#facing == Facing::North { return 5560; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 5582; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#facing == Facing::South { return 5581; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == false { return 5571; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::South { return 5584; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#facing == Facing::East { return 5622; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Top { return 5552; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::North { return 5563; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::OuterRight { return 5574; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South && self.r#waterlogged == false { return 5573; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::OuterRight { return 5575; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Top { return 5613; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 5548; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Top { return 5546; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 5554; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft { return 5572; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft { return 5602; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top { return 5609; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#facing == Facing::North { return 5550; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 5598; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == false { return 5621; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom { return 5580; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft { return 5608; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#facing == Facing::North { return 5561; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#facing == Facing::West && self.r#waterlogged == false { return 5595; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#half == Half::Top { return 5566; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Top { return 5551; }
        if self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::South { return 5569; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::InnerRight { return 5620; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 5549; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::West { return 5589; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft { return 5583; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5553 {
            return Some(CobblestoneStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 5591 {
            return Some(CobblestoneStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 5610 {
            return Some(CobblestoneStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 5588 {
            return Some(CobblestoneStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 5623 {
            return Some(CobblestoneStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 5570 {
            return Some(CobblestoneStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 5576 {
            return Some(CobblestoneStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 5592 {
            return Some(CobblestoneStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 5568 {
            return Some(CobblestoneStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 5564 {
            return Some(CobblestoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 5565 {
            return Some(CobblestoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 5577 {
            return Some(CobblestoneStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 5586 {
            return Some(CobblestoneStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 5599 {
            return Some(CobblestoneStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 5604 {
            return Some(CobblestoneStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 5585 {
            return Some(CobblestoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 5597 {
            return Some(CobblestoneStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 5606 {
            return Some(CobblestoneStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 5593 {
            return Some(CobblestoneStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 5578 {
            return Some(CobblestoneStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 5579 {
            return Some(CobblestoneStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 5590 {
            return Some(CobblestoneStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 5594 {
            return Some(CobblestoneStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 5617 {
            return Some(CobblestoneStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 5616 {
            return Some(CobblestoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 5619 {
            return Some(CobblestoneStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 5625 {
            return Some(CobblestoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 5605 {
            return Some(CobblestoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 5547 {
            return Some(CobblestoneStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 5596 {
            return Some(CobblestoneStairs {
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 5600 {
            return Some(CobblestoneStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 5611 {
            return Some(CobblestoneStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 5556 {
            return Some(CobblestoneStairs {
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 5624 {
            return Some(CobblestoneStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 5559 {
            return Some(CobblestoneStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 5618 {
            return Some(CobblestoneStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 5587 {
            return Some(CobblestoneStairs {
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 5558 {
            return Some(CobblestoneStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 5567 {
            return Some(CobblestoneStairs {
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 5601 {
            return Some(CobblestoneStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 5562 {
            return Some(CobblestoneStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 5603 {
            return Some(CobblestoneStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 5607 {
            return Some(CobblestoneStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 5555 {
            return Some(CobblestoneStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 5612 {
            return Some(CobblestoneStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 5614 {
            return Some(CobblestoneStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 5557 {
            return Some(CobblestoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 5615 {
            return Some(CobblestoneStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 5560 {
            return Some(CobblestoneStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 5582 {
            return Some(CobblestoneStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 5581 {
            return Some(CobblestoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 5571 {
            return Some(CobblestoneStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 5584 {
            return Some(CobblestoneStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 5622 {
            return Some(CobblestoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 5552 {
            return Some(CobblestoneStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 5563 {
            return Some(CobblestoneStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 5574 {
            return Some(CobblestoneStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 5573 {
            return Some(CobblestoneStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 5575 {
            return Some(CobblestoneStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 5613 {
            return Some(CobblestoneStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 5548 {
            return Some(CobblestoneStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 5546 {
            return Some(CobblestoneStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 5554 {
            return Some(CobblestoneStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 5572 {
            return Some(CobblestoneStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 5602 {
            return Some(CobblestoneStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 5609 {
            return Some(CobblestoneStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 5550 {
            return Some(CobblestoneStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
            });
        }
        if state_id == 5598 {
            return Some(CobblestoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 5621 {
            return Some(CobblestoneStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 5580 {
            return Some(CobblestoneStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 5608 {
            return Some(CobblestoneStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 5561 {
            return Some(CobblestoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
            });
        }
        if state_id == 5595 {
            return Some(CobblestoneStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 5566 {
            return Some(CobblestoneStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 5551 {
            return Some(CobblestoneStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 5569 {
            return Some(CobblestoneStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 5620 {
            return Some(CobblestoneStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 5549 {
            return Some(CobblestoneStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 5589 {
            return Some(CobblestoneStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 5583 {
            return Some(CobblestoneStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        return None;
    }
}

