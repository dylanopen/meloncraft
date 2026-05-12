use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleStairs {
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

impl BlockState for JungleStairs {
    fn to_id(self) -> i32 {
        if block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 9707; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight { return 9758; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true { return 9689; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 9713; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 9760; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom { return 9706; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false { return 9764; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight { return 9765; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft { return 9693; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false { return 9702; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 9716; }
        if block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 9697; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true { return 9701; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top { return 9711; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top { return 9708; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 9743; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 9691; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft { return 9714; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 9729; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 9730; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 9735; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom { return 9745; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft { return 9750; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::West { return 9727; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom { return 9704; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top { return 9696; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 9744; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::East { return 9749; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 9753; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 9754; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#shape == Shape::Straight { return 9757; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight { return 9766; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 9762; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 9761; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 9728; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true { return 9717; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom { return 9705; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterLeft { return 9694; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight { return 9722; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 9763; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom { return 9698; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::North { return 9699; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterLeft { return 9724; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 9703; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Top { return 9695; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::South { return 9719; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterRight { return 9755; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight { return 9687; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom { return 9739; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 9746; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::East { return 9747; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft { return 9700; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 9736; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false { return 9748; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 9734; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 9756; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 9759; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::West { return 9731; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false { return 9742; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 9733; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight { return 9738; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft { return 9710; }
        if block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West { return 9737; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 9732; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight { return 9718; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 9720; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight { return 9752; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 9690; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::South { return 9709; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 9740; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true { return 9715; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 9723; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 9741; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 9751; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 9725; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::North { return 9688; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 9726; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight { return 9692; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top { return 9712; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 9721; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9707 {
            return Some(JungleStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 9758 {
            return Some(JungleStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 9689 {
            return Some(JungleStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 9713 {
            return Some(JungleStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 9760 {
            return Some(JungleStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 9706 {
            return Some(JungleStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9764 {
            return Some(JungleStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 9765 {
            return Some(JungleStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 9693 {
            return Some(JungleStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 9702 {
            return Some(JungleStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 9716 {
            return Some(JungleStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 9697 {
            return Some(JungleStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 9701 {
            return Some(JungleStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 9711 {
            return Some(JungleStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 9708 {
            return Some(JungleStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 9743 {
            return Some(JungleStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 9691 {
            return Some(JungleStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 9714 {
            return Some(JungleStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 9729 {
            return Some(JungleStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 9730 {
            return Some(JungleStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 9735 {
            return Some(JungleStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 9745 {
            return Some(JungleStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9750 {
            return Some(JungleStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 9727 {
            return Some(JungleStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
            });
        }
        if state_id == 9704 {
            return Some(JungleStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9696 {
            return Some(JungleStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 9744 {
            return Some(JungleStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 9749 {
            return Some(JungleStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 9753 {
            return Some(JungleStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 9754 {
            return Some(JungleStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 9757 {
            return Some(JungleStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 9766 {
            return Some(JungleStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 9762 {
            return Some(JungleStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 9761 {
            return Some(JungleStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9728 {
            return Some(JungleStairs {
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 9717 {
            return Some(JungleStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 9705 {
            return Some(JungleStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9694 {
            return Some(JungleStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 9722 {
            return Some(JungleStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 9763 {
            return Some(JungleStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 9698 {
            return Some(JungleStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9699 {
            return Some(JungleStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 9724 {
            return Some(JungleStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 9703 {
            return Some(JungleStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 9695 {
            return Some(JungleStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 9719 {
            return Some(JungleStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 9755 {
            return Some(JungleStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 9687 {
            return Some(JungleStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 9739 {
            return Some(JungleStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9746 {
            return Some(JungleStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 9747 {
            return Some(JungleStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
            });
        }
        if state_id == 9700 {
            return Some(JungleStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 9736 {
            return Some(JungleStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 9748 {
            return Some(JungleStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 9734 {
            return Some(JungleStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 9756 {
            return Some(JungleStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 9759 {
            return Some(JungleStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9731 {
            return Some(JungleStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 9742 {
            return Some(JungleStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 9733 {
            return Some(JungleStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 9738 {
            return Some(JungleStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 9710 {
            return Some(JungleStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 9737 {
            return Some(JungleStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 9732 {
            return Some(JungleStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 9718 {
            return Some(JungleStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 9720 {
            return Some(JungleStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 9752 {
            return Some(JungleStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 9690 {
            return Some(JungleStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 9709 {
            return Some(JungleStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 9740 {
            return Some(JungleStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 9715 {
            return Some(JungleStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 9723 {
            return Some(JungleStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 9741 {
            return Some(JungleStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 9751 {
            return Some(JungleStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 9725 {
            return Some(JungleStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9688 {
            return Some(JungleStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
            });
        }
        if state_id == 9726 {
            return Some(JungleStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 9692 {
            return Some(JungleStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 9712 {
            return Some(JungleStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 9721 {
            return Some(JungleStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        return None;
    }
}

