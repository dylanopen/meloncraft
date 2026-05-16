use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkPrismarineStairs {
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

impl BlockState for DarkPrismarineStairs {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South && self.r#half == Half::Top { return 12618; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::OuterRight { return 12661; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#facing == Facing::East { return 12659; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::North { return 12606; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::OuterLeft { return 12639; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Top { return 12654; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom { return 12670; }
        if self.r#facing == Facing::West && self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#waterlogged == true { return 12636; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::OuterLeft { return 12649; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 12604; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft { return 12665; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::West { return 12642; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 12648; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::OuterRight { return 12671; }
        if self.r#facing == Facing::North && self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#half == Half::Top { return 12601; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Top { return 12593; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == false { return 12603; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#facing == Facing::North { return 12598; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::South { return 12623; }
        if self.r#facing == Facing::South && self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#half == Half::Top { return 12620; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#facing == Facing::South && self.r#waterlogged == false { return 12627; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Top { return 12637; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::Straight { return 12643; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 12644; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::InnerRight { return 12647; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#facing == Facing::West && self.r#waterlogged == true { return 12650; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft { return 12614; }
        if self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#waterlogged == true { return 12658; }
        if self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#waterlogged == false { return 12655; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::South { return 12619; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight { return 12631; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#facing == Facing::South { return 12622; }
        if self.r#facing == Facing::West && self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#waterlogged == false { return 12633; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 12638; }
        if self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#facing == Facing::East && self.r#half == Half::Top { return 12657; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#waterlogged == false { return 12667; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == false { return 12651; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#shape == Shape::OuterRight && self.r#half == Half::Top { return 12600; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top { return 12595; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 12616; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::OuterRight { return 12641; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 12605; }
        if self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::East { return 12664; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::Straight { return 12663; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 12668; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#facing == Facing::East { return 12669; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 12607; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == true { return 12608; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#facing == Facing::South && self.r#waterlogged == false { return 12621; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Top { return 12596; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::South { return 12625; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#shape == Shape::Straight { return 12632; }
        if self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#facing == Facing::South { return 12612; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 12592; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 12645; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 12624; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 12628; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::OuterRight { return 12640; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#shape == Shape::Straight && self.r#half == Half::Top { return 12652; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#facing == Facing::West { return 12634; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top { return 12599; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#facing == Facing::East && self.r#half == Half::Bottom { return 12666; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 12626; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 12610; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 12609; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft { return 12629; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::East { return 12653; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::InnerRight { return 12597; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#shape == Shape::OuterRight { return 12611; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::Straight { return 12602; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 12594; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#facing == Facing::West && self.r#waterlogged == true { return 12646; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == true { return 12660; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::Straight { return 12613; }
        if self.r#facing == Facing::East && self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#half == Half::Top { return 12656; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom { return 12630; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#facing == Facing::South { return 12617; }
        if self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#facing == Facing::West { return 12635; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 12615; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 12662; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12618 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 12661 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 12659 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 12606 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
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
        if state_id == 12654 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 12670 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12636 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
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
        if state_id == 12604 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12665 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12642 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 12648 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 12671 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 12601 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 12593 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 12603 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 12598 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 12623 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 12620 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 12627 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 12637 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 12643 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12644 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12647 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
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
        if state_id == 12614 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12658 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: true,
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
        if state_id == 12619 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 12631 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 12622 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
            });
        }
        if state_id == 12633 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 12638 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 12657 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 12667 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
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
        if state_id == 12600 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 12595 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 12616 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 12641 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 12605 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 12664 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 12663 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12668 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 12669 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::East,
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
        if state_id == 12608 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 12621 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 12596 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 12625 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 12632 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12612 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 12592 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 12645 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12624 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 12628 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 12640 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 12652 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 12634 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 12599 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
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
        if state_id == 12626 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 12610 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 12609 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12629 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 12653 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 12597 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 12611 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 12602 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12594 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 12646 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 12660 {
            return Some(DarkPrismarineStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 12613 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12656 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 12630 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12617 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 12635 {
            return Some(DarkPrismarineStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 12615 {
            return Some(DarkPrismarineStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 12662 {
            return Some(DarkPrismarineStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

