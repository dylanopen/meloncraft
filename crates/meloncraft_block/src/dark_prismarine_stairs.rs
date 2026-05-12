use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkPrismarineStairs {
    pub r#shape: Shape,
    pub waterlogged: bool,
    pub r#facing: Facing,
    pub r#half: Half,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Top,
    Bottom,
}

impl BlockState for DarkPrismarineStairs {
    fn to_id(self) -> i32 {
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight { return 12600; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom { return 12607; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft { return 12595; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false { return 12609; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterLeft { return 12618; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 12632; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::North && block_state.r#half == Half::Top { return 12597; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 12636; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterLeft { return 12649; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::East && block_state.r#half == Half::Top { return 12656; }
        if block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North { return 12603; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft { return 12608; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight { return 12612; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true { return 12658; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 12624; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom { return 12666; }
        if block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 12602; }
        if block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 12652; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false { return 12641; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 12669; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 12670; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true { return 12606; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 12613; }
        if block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 12601; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false { return 12633; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft { return 12599; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft { return 12635; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false { return 12619; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::East && block_state.r#half == Half::Top { return 12659; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight { return 12671; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 12620; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 12631; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft { return 12614; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#shape == Shape::Straight { return 12642; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight { return 12647; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 12593; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 12617; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft { return 12604; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 12664; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 12627; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight { return 12646; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top { return 12596; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 12645; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight { return 12616; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft { return 12598; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft { return 12634; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerRight { return 12657; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 12662; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 12630; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#shape == Shape::Straight { return 12622; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight { return 12626; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 12651; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 12663; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight { return 12623; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top { return 12638; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::West && block_state.r#half == Half::Top { return 12637; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterLeft { return 12639; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false { return 12629; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom { return 12605; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top && block_state.r#facing == Facing::East { return 12661; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight { return 12592; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 12665; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft { return 12654; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 12655; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 12628; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false { return 12643; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true { return 12594; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East { return 12667; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 12615; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top { return 12640; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterRight { return 12610; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight { return 12621; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false { return 12625; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::East { return 12660; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterLeft { return 12668; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 12650; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 12644; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::East && block_state.r#half == Half::Top { return 12653; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom { return 12648; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::North { return 12611; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12600 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 12607 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12595 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12609 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 12618 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 12632 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 12597 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 12636 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 12649 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 12656 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 12603 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 12608 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 12612 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12658 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 12624 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 12666 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12602 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 12652 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 12641 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 12669 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 12670 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12606 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 12613 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 12601 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 12633 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 12599 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 12635 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12619 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 12659 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 12671 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 12620 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 12631 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 12614 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12642 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12647 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 12593 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 12617 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 12604 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12664 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 12627 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 12646 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 12596 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 12645 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 12616 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 12598 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 12634 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12657 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 12662 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 12630 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12622 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12626 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 12651 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 12663 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12623 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12638 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 12637 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 12639 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 12629 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 12605 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12661 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 12592 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12665 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12654 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12655 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 12628 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12643 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 12594 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 12667 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 12615 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 12640 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 12610 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 12621 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 12625 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 12660 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 12668 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 12650 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 12644 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 12653 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 12648 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12611 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

