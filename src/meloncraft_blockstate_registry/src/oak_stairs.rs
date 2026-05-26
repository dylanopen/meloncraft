use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakStairs {
    pub r#facing: Facing,
    pub r#half: Half,
    pub r#shape: Shape,
    pub waterlogged: bool,
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

impl BlockState for OakStairs {
    fn to_id(&self) -> i32 {
        if self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 3719;
        }
        if self.r#shape == Shape::Straight
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
        {
            return 3746;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
        {
            return 3723;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
        {
            return 3743;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 3713;
        }
        if self.r#facing == Facing::North
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 3721;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
        {
            return 3708;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
        {
            return 3720;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
        {
            return 3755;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
        {
            return 3762;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
        {
            return 3752;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::West
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
        {
            return 3765;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 3710;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::North
        {
            return 3718;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
        {
            return 3735;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::North
        {
            return 3717;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
        {
            return 3724;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 3741;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
        {
            return 3757;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
        {
            return 3754;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
        {
            return 3731;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 3732;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
        {
            return 3740;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
        {
            return 3739;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
        {
            return 3763;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 3784;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
        {
            return 3738;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::North
        {
            return 3707;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
        {
            return 3733;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#shape == Shape::OuterLeft
        {
            return 3742;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 3712;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
        {
            return 3760;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
        {
            return 3769;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
        {
            return 3766;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 3728;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Bottom
        {
            return 3716;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
        {
            return 3722;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 3727;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::West
        {
            return 3747;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
        {
            return 3751;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 3756;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == false
        {
            return 3749;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == true
        {
            return 3726;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
        {
            return 3748;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#shape == Shape::InnerLeft
        {
            return 3768;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 3779;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 3775;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#shape == Shape::OuterRight
        {
            return 3734;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 3772;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 3783;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
        {
            return 3744;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == true
        {
            return 3736;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
        {
            return 3759;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
        {
            return 3785;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#shape == Shape::OuterLeft
        {
            return 3773;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
        {
            return 3729;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
        {
            return 3711;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Bottom
        {
            return 3778;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Bottom
        {
            return 3761;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
        {
            return 3780;
        }
        if self.r#shape == Shape::Straight
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
        {
            return 3706;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::North
        {
            return 3709;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#shape == Shape::OuterLeft
        {
            return 3753;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 3715;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#shape == Shape::Straight
        {
            return 3767;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
        {
            return 3737;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
        {
            return 3758;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
        {
            return 3764;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
        {
            return 3776;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
        {
            return 3750;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
        {
            return 3771;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#shape == Shape::OuterRight
        {
            return 3774;
        }
        if self.r#shape == Shape::Straight
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 3777;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
        {
            return 3730;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 3781;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#shape == Shape::OuterLeft
        {
            return 3782;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
        {
            return 3770;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
        {
            return 3725;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
        {
            return 3745;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
        {
            return 3714;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 3719 {
            return Some(OakStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 3746 {
            return Some(OakStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 3723 {
            return Some(OakStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 3743 {
            return Some(OakStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 3713 {
            return Some(OakStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 3721 {
            return Some(OakStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 3708 {
            return Some(OakStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 3720 {
            return Some(OakStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 3755 {
            return Some(OakStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 3762 {
            return Some(OakStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 3752 {
            return Some(OakStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 3765 {
            return Some(OakStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 3710 {
            return Some(OakStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 3718 {
            return Some(OakStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 3735 {
            return Some(OakStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 3717 {
            return Some(OakStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
            });
        }
        if state_id == 3724 {
            return Some(OakStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 3741 {
            return Some(OakStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 3757 {
            return Some(OakStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 3754 {
            return Some(OakStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 3731 {
            return Some(OakStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 3732 {
            return Some(OakStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 3740 {
            return Some(OakStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 3739 {
            return Some(OakStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 3763 {
            return Some(OakStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 3784 {
            return Some(OakStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 3738 {
            return Some(OakStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 3707 {
            return Some(OakStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
            });
        }
        if state_id == 3733 {
            return Some(OakStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 3742 {
            return Some(OakStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 3712 {
            return Some(OakStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 3760 {
            return Some(OakStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 3769 {
            return Some(OakStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 3766 {
            return Some(OakStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 3728 {
            return Some(OakStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 3716 {
            return Some(OakStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 3722 {
            return Some(OakStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 3727 {
            return Some(OakStairs {
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 3747 {
            return Some(OakStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
            });
        }
        if state_id == 3751 {
            return Some(OakStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 3756 {
            return Some(OakStairs {
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 3749 {
            return Some(OakStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 3726 {
            return Some(OakStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 3748 {
            return Some(OakStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 3768 {
            return Some(OakStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 3779 {
            return Some(OakStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 3775 {
            return Some(OakStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 3734 {
            return Some(OakStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 3772 {
            return Some(OakStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 3783 {
            return Some(OakStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 3744 {
            return Some(OakStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 3736 {
            return Some(OakStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 3759 {
            return Some(OakStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 3785 {
            return Some(OakStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 3773 {
            return Some(OakStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 3729 {
            return Some(OakStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 3711 {
            return Some(OakStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 3778 {
            return Some(OakStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 3761 {
            return Some(OakStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 3780 {
            return Some(OakStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 3706 {
            return Some(OakStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 3709 {
            return Some(OakStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 3753 {
            return Some(OakStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 3715 {
            return Some(OakStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 3767 {
            return Some(OakStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 3737 {
            return Some(OakStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 3758 {
            return Some(OakStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 3764 {
            return Some(OakStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 3776 {
            return Some(OakStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 3750 {
            return Some(OakStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 3771 {
            return Some(OakStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 3774 {
            return Some(OakStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 3777 {
            return Some(OakStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 3730 {
            return Some(OakStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 3781 {
            return Some(OakStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 3782 {
            return Some(OakStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 3770 {
            return Some(OakStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 3725 {
            return Some(OakStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 3745 {
            return Some(OakStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 3714 {
            return Some(OakStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
