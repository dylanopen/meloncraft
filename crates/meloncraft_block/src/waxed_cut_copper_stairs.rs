use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedCutCopperStairs {
    pub r#half: Half,
    pub r#shape: Shape,
    pub waterlogged: bool,
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

impl BlockState for WaxedCutCopperStairs {
    fn to_id(self) -> i32 {
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true { return 25767; }
        if block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::East { return 25777; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 25793; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 25779; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true { return 25787; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight { return 25751; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Top { return 25762; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight { return 25725; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom { return 25731; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft { return 25780; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false { return 25754; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 25769; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft { return 25794; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#shape == Shape::Straight { return 25757; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft { return 25760; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::North { return 25726; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterLeft { return 25784; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::East { return 25781; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 25740; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 25739; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true { return 25741; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight { return 25717; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::North && block_state.r#half == Half::Top { return 25721; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight { return 25782; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true { return 25791; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top { return 25719; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 25745; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 25736; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::West { return 25764; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight { return 25785; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 25783; }
        if block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West { return 25768; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false { return 25766; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East { return 25788; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight { return 25776; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom { return 25752; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 25758; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom { return 25749; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight { return 25727; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 25743; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 25728; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true { return 25735; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false { return 25732; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 25734; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 25774; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 25730; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom { return 25795; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 25750; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false { return 25790; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 25753; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft { return 25744; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight { return 25742; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 25775; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterLeft { return 25723; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 25755; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true { return 25759; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerRight { return 25771; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 25724; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 25756; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top { return 25718; }
        if block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South { return 25747; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight { return 25786; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 25746; }
        if block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 25737; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 25778; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 25738; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterLeft { return 25763; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 25792; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight { return 25722; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::West { return 25761; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft { return 25789; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom { return 25796; }
        if block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 25720; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerLeft { return 25770; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true { return 25733; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterLeft { return 25773; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top { return 25765; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 25748; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerRight { return 25772; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 25729; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25767 {
            return Some(WaxedCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 25777 {
            return Some(WaxedCutCopperStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 25793 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25779 {
            return Some(WaxedCutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 25787 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 25751 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25762 {
            return Some(WaxedCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 25725 {
            return Some(WaxedCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25731 {
            return Some(WaxedCutCopperStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25780 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25754 {
            return Some(WaxedCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 25769 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25794 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25757 {
            return Some(WaxedCutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25760 {
            return Some(WaxedCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25726 {
            return Some(WaxedCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 25784 {
            return Some(WaxedCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25781 {
            return Some(WaxedCutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 25740 {
            return Some(WaxedCutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 25739 {
            return Some(WaxedCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 25741 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 25717 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25721 {
            return Some(WaxedCutCopperStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 25782 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25791 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 25719 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 25745 {
            return Some(WaxedCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 25736 {
            return Some(WaxedCutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 25764 {
            return Some(WaxedCutCopperStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 25785 {
            return Some(WaxedCutCopperStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25783 {
            return Some(WaxedCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 25768 {
            return Some(WaxedCutCopperStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 25766 {
            return Some(WaxedCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 25788 {
            return Some(WaxedCutCopperStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 25776 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25752 {
            return Some(WaxedCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25758 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25749 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25727 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25743 {
            return Some(WaxedCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 25728 {
            return Some(WaxedCutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 25735 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 25732 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 25734 {
            return Some(WaxedCutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 25774 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25730 {
            return Some(WaxedCutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 25795 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25750 {
            return Some(WaxedCutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 25790 {
            return Some(WaxedCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 25753 {
            return Some(WaxedCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 25744 {
            return Some(WaxedCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25742 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25775 {
            return Some(WaxedCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25723 {
            return Some(WaxedCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25755 {
            return Some(WaxedCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 25759 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 25771 {
            return Some(WaxedCutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25724 {
            return Some(WaxedCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25756 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25718 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 25747 {
            return Some(WaxedCutCopperStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 25786 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25746 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25737 {
            return Some(WaxedCutCopperStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 25778 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25738 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 25763 {
            return Some(WaxedCutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25792 {
            return Some(WaxedCutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 25722 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25761 {
            return Some(WaxedCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 25789 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25796 {
            return Some(WaxedCutCopperStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25720 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 25770 {
            return Some(WaxedCutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25733 {
            return Some(WaxedCutCopperStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 25773 {
            return Some(WaxedCutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25765 {
            return Some(WaxedCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 25748 {
            return Some(WaxedCutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 25772 {
            return Some(WaxedCutCopperStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25729 {
            return Some(WaxedCutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

