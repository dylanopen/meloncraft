use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmoothSandstoneStairs {
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

impl BlockState for SmoothSandstoneStairs {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
        {
            return 15711;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
        {
            return 15712;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
        {
            return 15719;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 15727;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
        {
            return 15670;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#shape == Shape::Straight
        {
            return 15685;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 15723;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
        {
            return 15686;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 15713;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Bottom
        {
            return 15709;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
        {
            return 15676;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#shape == Shape::InnerLeft
        {
            return 15677;
        }
        if self.r#shape == Shape::Straight
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 15655;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
        {
            return 15728;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::Straight
        {
            return 15725;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
        {
            return 15706;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
        {
            return 15673;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 15688;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#shape == Shape::OuterRight
        {
            return 15672;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 15717;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
        {
            return 15661;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
        {
            return 15656;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
        {
            return 15716;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
        {
            return 15658;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 15733;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
        {
            return 15689;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
        {
            return 15690;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 15696;
        }
        if self.r#shape == Shape::Straight
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 15714;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#shape == Shape::OuterLeft
        {
            return 15720;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
        {
            return 15682;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
        {
            return 15721;
        }
        if self.r#facing == Facing::North
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 15668;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
        {
            return 15729;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 15675;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 15693;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::Straight
        {
            return 15674;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::North
        {
            return 15660;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 15691;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
        {
            return 15697;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#shape == Shape::Straight
        {
            return 15695;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 15687;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
        {
            return 15731;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 15698;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
        {
            return 15700;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::East
        {
            return 15726;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#shape == Shape::InnerRight
        {
            return 15679;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
        {
            return 15680;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
        {
            return 15692;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
        {
            return 15722;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
        {
            return 15681;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 15683;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 15665;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
        {
            return 15671;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 15657;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
        {
            return 15732;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
        {
            return 15708;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
        {
            return 15718;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Bottom
        {
            return 15724;
        }
        if self.r#shape == Shape::Straight
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 15694;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Top
        {
            return 15662;
        }
        if self.r#shape == Shape::Straight
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
        {
            return 15664;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 15707;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::South
        {
            return 15684;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 15699;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::North
        {
            return 15666;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
        {
            return 15659;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
        {
            return 15678;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
        {
            return 15730;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 15703;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
        {
            return 15663;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#shape == Shape::InnerLeft
        {
            return 15667;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 15702;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
        {
            return 15715;
        }
        if self.r#shape == Shape::Straight
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 15705;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
        {
            return 15669;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == true
        {
            return 15704;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
        {
            return 15710;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == true
        {
            return 15654;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
        {
            return 15701;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15711 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 15712 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15719 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 15727 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 15670 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 15685 {
            return Some(SmoothSandstoneStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15723 {
            return Some(SmoothSandstoneStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 15686 {
            return Some(SmoothSandstoneStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 15713 {
            return Some(SmoothSandstoneStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15709 {
            return Some(SmoothSandstoneStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15676 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 15677 {
            return Some(SmoothSandstoneStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15655 {
            return Some(SmoothSandstoneStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 15728 {
            return Some(SmoothSandstoneStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 15725 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15706 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 15673 {
            return Some(SmoothSandstoneStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 15688 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 15672 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15717 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 15661 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 15656 {
            return Some(SmoothSandstoneStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 15716 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15658 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15733 {
            return Some(SmoothSandstoneStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15689 {
            return Some(SmoothSandstoneStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15690 {
            return Some(SmoothSandstoneStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 15696 {
            return Some(SmoothSandstoneStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 15714 {
            return Some(SmoothSandstoneStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 15720 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15682 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15721 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 15668 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15729 {
            return Some(SmoothSandstoneStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15675 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 15693 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15674 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15660 {
            return Some(SmoothSandstoneStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 15691 {
            return Some(SmoothSandstoneStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 15697 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15695 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15687 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 15731 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 15698 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 15700 {
            return Some(SmoothSandstoneStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 15726 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 15679 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15680 {
            return Some(SmoothSandstoneStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 15692 {
            return Some(SmoothSandstoneStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15722 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15681 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 15683 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 15665 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 15671 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15657 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 15732 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15708 {
            return Some(SmoothSandstoneStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 15718 {
            return Some(SmoothSandstoneStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 15724 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15694 {
            return Some(SmoothSandstoneStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 15662 {
            return Some(SmoothSandstoneStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 15664 {
            return Some(SmoothSandstoneStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 15707 {
            return Some(SmoothSandstoneStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 15684 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
            });
        }
        if state_id == 15699 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 15666 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 15659 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15678 {
            return Some(SmoothSandstoneStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 15730 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 15703 {
            return Some(SmoothSandstoneStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 15663 {
            return Some(SmoothSandstoneStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15667 {
            return Some(SmoothSandstoneStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15702 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 15715 {
            return Some(SmoothSandstoneStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 15705 {
            return Some(SmoothSandstoneStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 15669 {
            return Some(SmoothSandstoneStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 15704 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 15710 {
            return Some(SmoothSandstoneStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15654 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 15701 {
            return Some(SmoothSandstoneStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        return None;
    }
}
