use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurpurStairs {
    pub waterlogged: bool,
    pub r#shape: Shape,
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

impl BlockState for PurpurStairs {
    fn to_id(&self) -> i32 {
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#shape == Shape::OuterRight
        {
            return 14573;
        }
        if self.r#facing == Facing::North
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 14530;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
        {
            return 14555;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
        {
            return 14569;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 14571;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 14537;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#shape == Shape::InnerRight
        {
            return 14529;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 14540;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
        {
            return 14518;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::West
        {
            return 14570;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Top
        {
            return 14560;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
        {
            return 14536;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 14589;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
        {
            return 14549;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
        {
            return 14515;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
        {
            return 14532;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
        {
            return 14565;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 14552;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::East
        {
            return 14579;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#shape == Shape::OuterRight
        {
            return 14582;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 14557;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 14586;
        }
        if self.r#facing == Facing::North
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 14531;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::South
        {
            return 14538;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
        {
            return 14567;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
        {
            return 14543;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::East
        {
            return 14577;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 14578;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
        {
            return 14559;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Bottom
        {
            return 14568;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 14548;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 14566;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
        {
            return 14591;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 14593;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::South
        {
            return 14547;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
        {
            return 14545;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterRight
        {
            return 14523;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 14542;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
        {
            return 14575;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
        {
            return 14563;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
        {
            return 14551;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
        {
            return 14534;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#shape == Shape::InnerLeft
        {
            return 14556;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#shape == Shape::Straight
        {
            return 14554;
        }
        if self.r#shape == Shape::Straight
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Top
        {
            return 14574;
        }
        if self.r#facing == Facing::North
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 14519;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
        {
            return 14546;
        }
        if self.r#shape == Shape::Straight
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 14535;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
        {
            return 14588;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
        {
            return 14561;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 14592;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 14583;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
        {
            return 14587;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#shape == Shape::Straight
        {
            return 14514;
        }
        if self.r#shape == Shape::Straight
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
        {
            return 14524;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
        {
            return 14528;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
        {
            return 14580;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
        {
            return 14517;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Bottom
        {
            return 14550;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 14562;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
        {
            return 14539;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
        {
            return 14581;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::North
        {
            return 14525;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
        {
            return 14526;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
        {
            return 14522;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
        {
            return 14544;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Bottom
        {
            return 14564;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == true
        {
            return 14584;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
        {
            return 14520;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
        {
            return 14516;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
        {
            return 14521;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
        {
            return 14527;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterRight
        {
            return 14533;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::South
        {
            return 14541;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::West
        {
            return 14558;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
        {
            return 14572;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::Straight
        {
            return 14585;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::East
        {
            return 14590;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
        {
            return 14576;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 14553;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14573 {
            return Some(PurpurStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 14530 {
            return Some(PurpurStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 14555 {
            return Some(PurpurStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 14569 {
            return Some(PurpurStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 14571 {
            return Some(PurpurStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 14537 {
            return Some(PurpurStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 14529 {
            return Some(PurpurStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 14540 {
            return Some(PurpurStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 14518 {
            return Some(PurpurStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 14570 {
            return Some(PurpurStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 14560 {
            return Some(PurpurStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 14536 {
            return Some(PurpurStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 14589 {
            return Some(PurpurStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 14549 {
            return Some(PurpurStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 14515 {
            return Some(PurpurStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 14532 {
            return Some(PurpurStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 14565 {
            return Some(PurpurStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 14552 {
            return Some(PurpurStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 14579 {
            return Some(PurpurStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 14582 {
            return Some(PurpurStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 14557 {
            return Some(PurpurStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 14586 {
            return Some(PurpurStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 14531 {
            return Some(PurpurStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 14538 {
            return Some(PurpurStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
            });
        }
        if state_id == 14567 {
            return Some(PurpurStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 14543 {
            return Some(PurpurStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 14577 {
            return Some(PurpurStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 14578 {
            return Some(PurpurStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 14559 {
            return Some(PurpurStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 14568 {
            return Some(PurpurStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 14548 {
            return Some(PurpurStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 14566 {
            return Some(PurpurStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 14591 {
            return Some(PurpurStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 14593 {
            return Some(PurpurStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 14547 {
            return Some(PurpurStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 14545 {
            return Some(PurpurStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 14523 {
            return Some(PurpurStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 14542 {
            return Some(PurpurStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 14575 {
            return Some(PurpurStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 14563 {
            return Some(PurpurStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 14551 {
            return Some(PurpurStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 14534 {
            return Some(PurpurStairs {
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 14556 {
            return Some(PurpurStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 14554 {
            return Some(PurpurStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 14574 {
            return Some(PurpurStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 14519 {
            return Some(PurpurStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 14546 {
            return Some(PurpurStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 14535 {
            return Some(PurpurStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 14588 {
            return Some(PurpurStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 14561 {
            return Some(PurpurStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 14592 {
            return Some(PurpurStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 14583 {
            return Some(PurpurStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 14587 {
            return Some(PurpurStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 14514 {
            return Some(PurpurStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 14524 {
            return Some(PurpurStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 14528 {
            return Some(PurpurStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 14580 {
            return Some(PurpurStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 14517 {
            return Some(PurpurStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 14550 {
            return Some(PurpurStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 14562 {
            return Some(PurpurStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 14539 {
            return Some(PurpurStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 14581 {
            return Some(PurpurStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 14525 {
            return Some(PurpurStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
            });
        }
        if state_id == 14526 {
            return Some(PurpurStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 14522 {
            return Some(PurpurStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 14544 {
            return Some(PurpurStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 14564 {
            return Some(PurpurStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 14584 {
            return Some(PurpurStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 14520 {
            return Some(PurpurStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 14516 {
            return Some(PurpurStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 14521 {
            return Some(PurpurStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 14527 {
            return Some(PurpurStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 14533 {
            return Some(PurpurStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 14541 {
            return Some(PurpurStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 14558 {
            return Some(PurpurStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 14572 {
            return Some(PurpurStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 14585 {
            return Some(PurpurStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 14590 {
            return Some(PurpurStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 14576 {
            return Some(PurpurStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 14553 {
            return Some(PurpurStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        return None;
    }
}
