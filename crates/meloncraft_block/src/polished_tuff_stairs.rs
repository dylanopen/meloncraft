use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedTuffStairs {
    pub r#shape: Shape,
    pub r#facing: Facing,
    pub r#half: Half,
    pub waterlogged: bool,
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

impl BlockState for PolishedTuffStairs {
    fn to_id(&self) -> i32 {
        if self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#facing == Facing::West { return 23715; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#facing == Facing::East { return 23728; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 23721; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#facing == Facing::North { return 23677; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::OuterRight { return 23747; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#waterlogged == false { return 23723; }
        if self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == true { return 23690; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft { return 23670; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Bottom { return 23685; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#shape == Shape::OuterRight { return 23697; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::West { return 23727; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == true { return 23738; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == false { return 23699; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#facing == Facing::West && self.r#waterlogged == false { return 23713; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft { return 23741; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 23683; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == false { return 23669; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::South && self.r#waterlogged == false { return 23691; }
        if self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom && self.r#waterlogged == true { return 23700; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight { return 23706; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Top { return 23731; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::OuterRight { return 23746; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 23712; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::North { return 23668; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#half == Half::Top { return 23676; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 23726; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft { return 23710; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft { return 23745; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#shape == Shape::InnerRight { return 23703; }
        if self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#waterlogged == false { return 23735; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == true { return 23680; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#facing == Facing::West { return 23718; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#half == Half::Top { return 23695; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#facing == Facing::West { return 23714; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 23720; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South { return 23694; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::InnerRight { return 23672; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Top { return 23717; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft { return 23684; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 23708; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 23705; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft { return 23740; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::North { return 23678; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft { return 23674; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Top { return 23675; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 23687; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == true { return 23734; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#facing == Facing::West && self.r#waterlogged == true { return 23716; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#shape == Shape::InnerRight { return 23702; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 23724; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft { return 23730; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::East { return 23743; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == false { return 23737; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 23688; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::InnerRight { return 23673; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#waterlogged == false { return 23729; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#facing == Facing::West && self.r#waterlogged == true { return 23722; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::North { return 23681; }
        if self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#facing == Facing::South && self.r#waterlogged == false { return 23689; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#half == Half::Bottom { return 23704; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft { return 23711; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom { return 23701; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#facing == Facing::East { return 23742; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::Straight { return 23739; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight { return 23686; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 23707; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#facing == Facing::North { return 23682; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::Straight && self.r#facing == Facing::South { return 23698; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 23736; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::InnerRight && self.r#half == Half::Top { return 23733; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::North { return 23671; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 23692; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#half == Half::Top { return 23732; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Bottom { return 23679; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::West { return 23725; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 23709; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 23744; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#facing == Facing::South && self.r#waterlogged == false { return 23693; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::Straight && self.r#half == Half::Bottom { return 23719; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::OuterRight { return 23696; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23715 {
            return Some(PolishedTuffStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 23728 {
            return Some(PolishedTuffStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
            });
        }
        if state_id == 23721 {
            return Some(PolishedTuffStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 23677 {
            return Some(PolishedTuffStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 23747 {
            return Some(PolishedTuffStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 23723 {
            return Some(PolishedTuffStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 23690 {
            return Some(PolishedTuffStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 23670 {
            return Some(PolishedTuffStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 23685 {
            return Some(PolishedTuffStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 23697 {
            return Some(PolishedTuffStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 23727 {
            return Some(PolishedTuffStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 23738 {
            return Some(PolishedTuffStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 23699 {
            return Some(PolishedTuffStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 23713 {
            return Some(PolishedTuffStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 23741 {
            return Some(PolishedTuffStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 23683 {
            return Some(PolishedTuffStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 23669 {
            return Some(PolishedTuffStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 23691 {
            return Some(PolishedTuffStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 23700 {
            return Some(PolishedTuffStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 23706 {
            return Some(PolishedTuffStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 23731 {
            return Some(PolishedTuffStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 23746 {
            return Some(PolishedTuffStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 23712 {
            return Some(PolishedTuffStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 23668 {
            return Some(PolishedTuffStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 23676 {
            return Some(PolishedTuffStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 23726 {
            return Some(PolishedTuffStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 23710 {
            return Some(PolishedTuffStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 23745 {
            return Some(PolishedTuffStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 23703 {
            return Some(PolishedTuffStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 23735 {
            return Some(PolishedTuffStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 23680 {
            return Some(PolishedTuffStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 23718 {
            return Some(PolishedTuffStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 23695 {
            return Some(PolishedTuffStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 23714 {
            return Some(PolishedTuffStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 23720 {
            return Some(PolishedTuffStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 23694 {
            return Some(PolishedTuffStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 23672 {
            return Some(PolishedTuffStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 23717 {
            return Some(PolishedTuffStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 23684 {
            return Some(PolishedTuffStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 23708 {
            return Some(PolishedTuffStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 23705 {
            return Some(PolishedTuffStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 23740 {
            return Some(PolishedTuffStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 23678 {
            return Some(PolishedTuffStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 23674 {
            return Some(PolishedTuffStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 23675 {
            return Some(PolishedTuffStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 23687 {
            return Some(PolishedTuffStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 23734 {
            return Some(PolishedTuffStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 23716 {
            return Some(PolishedTuffStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 23702 {
            return Some(PolishedTuffStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 23724 {
            return Some(PolishedTuffStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 23730 {
            return Some(PolishedTuffStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 23743 {
            return Some(PolishedTuffStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 23737 {
            return Some(PolishedTuffStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 23688 {
            return Some(PolishedTuffStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 23673 {
            return Some(PolishedTuffStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 23729 {
            return Some(PolishedTuffStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 23722 {
            return Some(PolishedTuffStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 23681 {
            return Some(PolishedTuffStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 23689 {
            return Some(PolishedTuffStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 23704 {
            return Some(PolishedTuffStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 23711 {
            return Some(PolishedTuffStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 23701 {
            return Some(PolishedTuffStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 23742 {
            return Some(PolishedTuffStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 23739 {
            return Some(PolishedTuffStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 23686 {
            return Some(PolishedTuffStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 23707 {
            return Some(PolishedTuffStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 23682 {
            return Some(PolishedTuffStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
            });
        }
        if state_id == 23698 {
            return Some(PolishedTuffStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
            });
        }
        if state_id == 23736 {
            return Some(PolishedTuffStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 23733 {
            return Some(PolishedTuffStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 23671 {
            return Some(PolishedTuffStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 23692 {
            return Some(PolishedTuffStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 23732 {
            return Some(PolishedTuffStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 23679 {
            return Some(PolishedTuffStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 23725 {
            return Some(PolishedTuffStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 23709 {
            return Some(PolishedTuffStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 23744 {
            return Some(PolishedTuffStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 23693 {
            return Some(PolishedTuffStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 23719 {
            return Some(PolishedTuffStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 23696 {
            return Some(PolishedTuffStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
            });
        }
        return None;
    }
}

