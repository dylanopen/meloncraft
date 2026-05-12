use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmoothQuartzStairs {
    pub r#half: Half,
    pub waterlogged: bool,
    pub r#shape: Shape,
    pub r#facing: Facing,
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

impl BlockState for SmoothQuartzStairs {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false { return 15813; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 15812; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 15736; }
        if block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 15751; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top && block_state.r#facing == Facing::East { return 15803; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::North { return 15738; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::West { return 15774; }
        if block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East { return 15804; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 15754; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft { return 15776; }
        if block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 15805; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 15765; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false { return 15781; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight { return 15758; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight { return 15782; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 15744; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 15767; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom { return 15773; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top { return 15794; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 15761; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom { return 15750; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 15785; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 15790; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true { return 15770; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 15771; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft { return 15806; }
        if block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 15737; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 15755; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top && block_state.r#facing == Facing::South { return 15763; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight { return 15739; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 15788; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 15783; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 15802; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 15807; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false { return 15777; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterLeft { return 15791; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft { return 15746; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight { return 15752; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft { return 15766; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 15786; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 15798; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#shape == Shape::Straight { return 15735; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom { return 15787; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 15789; }
        if block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 15795; }
        if block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 15775; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom { return 15753; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top { return 15797; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterLeft { return 15741; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterLeft { return 15810; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 15760; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight { return 15762; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true { return 15772; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight { return 15778; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 15759; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::North { return 15742; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterLeft { return 15801; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 15784; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 15811; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 15769; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 15793; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerRight { return 15799; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight { return 15745; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 15749; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 15796; }
        if block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 15740; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true { return 15768; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true { return 15800; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 15756; }
        if block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 15748; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterRight { return 15792; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Top { return 15743; }
        if block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 15764; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 15747; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft { return 15780; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::East { return 15808; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top { return 15757; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight { return 15734; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false { return 15809; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 15779; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15813 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 15812 {
            return Some(SmoothQuartzStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 15736 {
            return Some(SmoothQuartzStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 15751 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15803 {
            return Some(SmoothQuartzStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 15738 {
            return Some(SmoothQuartzStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
            });
        }
        if state_id == 15774 {
            return Some(SmoothQuartzStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
            });
        }
        if state_id == 15804 {
            return Some(SmoothQuartzStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 15754 {
            return Some(SmoothQuartzStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 15776 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15805 {
            return Some(SmoothQuartzStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15765 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15781 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 15758 {
            return Some(SmoothQuartzStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15782 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15744 {
            return Some(SmoothQuartzStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 15767 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15773 {
            return Some(SmoothQuartzStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15794 {
            return Some(SmoothQuartzStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 15761 {
            return Some(SmoothQuartzStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 15750 {
            return Some(SmoothQuartzStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15785 {
            return Some(SmoothQuartzStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 15790 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 15770 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 15771 {
            return Some(SmoothQuartzStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 15806 {
            return Some(SmoothQuartzStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15737 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 15755 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 15763 {
            return Some(SmoothQuartzStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 15739 {
            return Some(SmoothQuartzStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15788 {
            return Some(SmoothQuartzStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 15783 {
            return Some(SmoothQuartzStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 15802 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 15807 {
            return Some(SmoothQuartzStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 15777 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 15791 {
            return Some(SmoothQuartzStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15746 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15752 {
            return Some(SmoothQuartzStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15766 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15786 {
            return Some(SmoothQuartzStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 15798 {
            return Some(SmoothQuartzStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 15735 {
            return Some(SmoothQuartzStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15787 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15789 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 15795 {
            return Some(SmoothQuartzStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 15775 {
            return Some(SmoothQuartzStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 15753 {
            return Some(SmoothQuartzStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15797 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 15741 {
            return Some(SmoothQuartzStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15810 {
            return Some(SmoothQuartzStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15760 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 15762 {
            return Some(SmoothQuartzStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15772 {
            return Some(SmoothQuartzStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15778 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15759 {
            return Some(SmoothQuartzStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 15742 {
            return Some(SmoothQuartzStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 15801 {
            return Some(SmoothQuartzStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15784 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 15811 {
            return Some(SmoothQuartzStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15769 {
            return Some(SmoothQuartzStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15793 {
            return Some(SmoothQuartzStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 15799 {
            return Some(SmoothQuartzStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15745 {
            return Some(SmoothQuartzStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15749 {
            return Some(SmoothQuartzStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 15796 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 15740 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 15768 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15800 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 15756 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 15748 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 15792 {
            return Some(SmoothQuartzStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15743 {
            return Some(SmoothQuartzStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 15764 {
            return Some(SmoothQuartzStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 15747 {
            return Some(SmoothQuartzStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 15780 {
            return Some(SmoothQuartzStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15808 {
            return Some(SmoothQuartzStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 15757 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 15734 {
            return Some(SmoothQuartzStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15809 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 15779 {
            return Some(SmoothQuartzStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

