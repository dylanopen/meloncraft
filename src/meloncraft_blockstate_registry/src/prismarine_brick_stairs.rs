use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrismarineBrickStairs {
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

impl BlockState for PrismarineBrickStairs {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
        {
            return 12512;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#shape == Shape::Straight
        {
            return 12582;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
        {
            return 12549;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 12524;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 12528;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#shape == Shape::OuterRight
        {
            return 12561;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
        {
            return 12523;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
        {
            return 12562;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::Straight
        {
            return 12563;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterRight
        {
            return 12541;
        }
        if self.r#shape == Shape::Straight
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Top
        {
            return 12572;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
        {
            return 12546;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
        {
            return 12522;
        }
        if self.r#shape == Shape::Straight
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
        {
            return 12533;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
        {
            return 12550;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
        {
            return 12555;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
        {
            return 12568;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
        {
            return 12571;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
        {
            return 12589;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::North
        {
            return 12514;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::North
            && self.r#shape == Shape::OuterLeft
        {
            return 12518;
        }
        if self.r#shape == Shape::Straight
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 12542;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Top
        {
            return 12581;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#shape == Shape::Straight
        {
            return 12553;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
        {
            return 12576;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::West
        {
            return 12557;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
        {
            return 12529;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
        {
            return 12535;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
        {
            return 12584;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 12569;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 12534;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
        {
            return 12530;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#shape == Shape::Straight
        {
            return 12552;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 12577;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
        {
            return 12516;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
        {
            return 12527;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
        {
            return 12526;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Bottom
        {
            return 12525;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
        {
            return 12556;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 12583;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::East
        {
            return 12588;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
        {
            return 12520;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Top
        {
            return 12539;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#shape == Shape::InnerLeft
        {
            return 12574;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 12587;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
        {
            return 12564;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
        {
            return 12586;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::North
            && self.r#shape == Shape::InnerLeft
        {
            return 12515;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
        {
            return 12513;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Top
        {
            return 12559;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 12519;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
        {
            return 12554;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#shape == Shape::InnerRight
        {
            return 12536;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
        {
            return 12544;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 12551;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#shape == Shape::OuterRight
        {
            return 12591;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
        {
            return 12547;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
        {
            return 12573;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 12543;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
        {
            return 12570;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
        {
            return 12566;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 12567;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#half == Half::Top
        {
            return 12579;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
        {
            return 12580;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
        {
            return 12537;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::West
        {
            return 12565;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterRight
        {
            return 12521;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::East
        {
            return 12575;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#shape == Shape::OuterRight
        {
            return 12540;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Top
        {
            return 12578;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
        {
            return 12548;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
        {
            return 12558;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#shape == Shape::Straight
        {
            return 12532;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
        {
            return 12517;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
        {
            return 12560;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#shape == Shape::OuterRight
        {
            return 12531;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 12590;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
        {
            return 12538;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#shape == Shape::InnerLeft
        {
            return 12585;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == false
        {
            return 12545;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12512 {
            return Some(PrismarineBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 12582 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12549 {
            return Some(PrismarineBrickStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 12524 {
            return Some(PrismarineBrickStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12528 {
            return Some(PrismarineBrickStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12561 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 12523 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 12562 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 12563 {
            return Some(PrismarineBrickStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12541 {
            return Some(PrismarineBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 12572 {
            return Some(PrismarineBrickStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 12546 {
            return Some(PrismarineBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 12522 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 12533 {
            return Some(PrismarineBrickStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 12550 {
            return Some(PrismarineBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12555 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 12568 {
            return Some(PrismarineBrickStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 12571 {
            return Some(PrismarineBrickStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12589 {
            return Some(PrismarineBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 12514 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 12518 {
            return Some(PrismarineBrickStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 12542 {
            return Some(PrismarineBrickStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12581 {
            return Some(PrismarineBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 12553 {
            return Some(PrismarineBrickStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12576 {
            return Some(PrismarineBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 12557 {
            return Some(PrismarineBrickStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 12529 {
            return Some(PrismarineBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 12535 {
            return Some(PrismarineBrickStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 12584 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 12569 {
            return Some(PrismarineBrickStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12534 {
            return Some(PrismarineBrickStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 12530 {
            return Some(PrismarineBrickStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 12552 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12577 {
            return Some(PrismarineBrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 12516 {
            return Some(PrismarineBrickStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 12527 {
            return Some(PrismarineBrickStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12526 {
            return Some(PrismarineBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12525 {
            return Some(PrismarineBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12556 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 12583 {
            return Some(PrismarineBrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 12588 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 12520 {
            return Some(PrismarineBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 12539 {
            return Some(PrismarineBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 12574 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12587 {
            return Some(PrismarineBrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12564 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 12586 {
            return Some(PrismarineBrickStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 12515 {
            return Some(PrismarineBrickStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12513 {
            return Some(PrismarineBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 12559 {
            return Some(PrismarineBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 12519 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 12554 {
            return Some(PrismarineBrickStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 12536 {
            return Some(PrismarineBrickStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 12544 {
            return Some(PrismarineBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 12551 {
            return Some(PrismarineBrickStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12591 {
            return Some(PrismarineBrickStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 12547 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 12573 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 12543 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 12570 {
            return Some(PrismarineBrickStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12566 {
            return Some(PrismarineBrickStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12567 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 12579 {
            return Some(PrismarineBrickStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 12580 {
            return Some(PrismarineBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 12537 {
            return Some(PrismarineBrickStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 12565 {
            return Some(PrismarineBrickStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 12521 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 12575 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 12540 {
            return Some(PrismarineBrickStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 12578 {
            return Some(PrismarineBrickStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 12548 {
            return Some(PrismarineBrickStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12558 {
            return Some(PrismarineBrickStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 12532 {
            return Some(PrismarineBrickStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12517 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 12560 {
            return Some(PrismarineBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 12531 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 12590 {
            return Some(PrismarineBrickStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 12538 {
            return Some(PrismarineBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 12585 {
            return Some(PrismarineBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12545 {
            return Some(PrismarineBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        return None;
    }
}
