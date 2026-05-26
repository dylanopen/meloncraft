use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeepslateTileStairs {
    pub r#half: Half,
    pub r#facing: Facing,
    pub r#shape: Shape,
    pub waterlogged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Top,
    Bottom,
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

impl BlockState for DeepslateTileStairs {
    fn to_id(&self) -> i32 {
        if self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 28561;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
        {
            return 28560;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
        {
            return 28562;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 28601;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::North
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
        {
            return 28549;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
        {
            return 28618;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 28620;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
        {
            return 28571;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 28622;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
        {
            return 28594;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#shape == Shape::OuterRight
        {
            return 28585;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 28589;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 28605;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
        {
            return 28626;
        }
        if self.r#shape == Shape::Straight
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 28558;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::West
        {
            return 28606;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
        {
            return 28551;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
        {
            return 28555;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::South
        {
            return 28584;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
        {
            return 28595;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 28591;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
        {
            return 28604;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
        {
            return 28573;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
        {
            return 28565;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
        {
            return 28623;
        }
        if self.r#shape == Shape::Straight
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
        {
            return 28557;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 28615;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Bottom
        {
            return 28580;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
        {
            return 28608;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
        {
            return 28612;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Bottom
        {
            return 28602;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
        {
            return 28616;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#shape == Shape::InnerRight
        {
            return 28572;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::West
        {
            return 28598;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == false
        {
            return 28600;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == false
        {
            return 28570;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
        {
            return 28625;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::South
        {
            return 28577;
        }
        if self.r#shape == Shape::Straight
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
        {
            return 28597;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
        {
            return 28590;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::North
        {
            return 28554;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
        {
            return 28566;
        }
        if self.r#facing == Facing::North
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 28553;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
        {
            return 28569;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
        {
            return 28576;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#shape == Shape::InnerLeft
        {
            return 28550;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterRight
        {
            return 28586;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
        {
            return 28575;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
        {
            return 28581;
        }
        if self.r#shape == Shape::Straight
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
        {
            return 28587;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
        {
            return 28593;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 28599;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::East
        {
            return 28617;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
        {
            return 28559;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
        {
            return 28564;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 28613;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
        {
            return 28611;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::South
        {
            return 28578;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#shape == Shape::OuterRight
        {
            return 28556;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
        {
            return 28552;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
        {
            return 28548;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
        {
            return 28579;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
        {
            return 28582;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
        {
            return 28610;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
        {
            return 28624;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 28592;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 28574;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
        {
            return 28588;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#shape == Shape::OuterRight
        {
            return 28596;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
        {
            return 28614;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
        {
            return 28583;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::North
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == true
        {
            return 28547;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Bottom
        {
            return 28563;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#shape == Shape::Straight
        {
            return 28567;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::Straight
        {
            return 28607;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::East
            && self.r#half == Half::Top
        {
            return 28609;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 28619;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
        {
            return 28568;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 28603;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::East
        {
            return 28621;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 28561 {
            return Some(DeepslateTileStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 28560 {
            return Some(DeepslateTileStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 28562 {
            return Some(DeepslateTileStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 28601 {
            return Some(DeepslateTileStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 28549 {
            return Some(DeepslateTileStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 28618 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 28620 {
            return Some(DeepslateTileStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 28571 {
            return Some(DeepslateTileStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 28622 {
            return Some(DeepslateTileStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 28594 {
            return Some(DeepslateTileStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 28585 {
            return Some(DeepslateTileStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 28589 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 28605 {
            return Some(DeepslateTileStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 28626 {
            return Some(DeepslateTileStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 28558 {
            return Some(DeepslateTileStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 28606 {
            return Some(DeepslateTileStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 28551 {
            return Some(DeepslateTileStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 28555 {
            return Some(DeepslateTileStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 28584 {
            return Some(DeepslateTileStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 28595 {
            return Some(DeepslateTileStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 28591 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 28604 {
            return Some(DeepslateTileStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 28573 {
            return Some(DeepslateTileStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 28565 {
            return Some(DeepslateTileStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 28623 {
            return Some(DeepslateTileStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 28557 {
            return Some(DeepslateTileStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 28615 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 28580 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 28608 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 28612 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 28602 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 28616 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 28572 {
            return Some(DeepslateTileStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 28598 {
            return Some(DeepslateTileStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 28600 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 28570 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 28625 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 28577 {
            return Some(DeepslateTileStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
            });
        }
        if state_id == 28597 {
            return Some(DeepslateTileStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 28590 {
            return Some(DeepslateTileStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 28554 {
            return Some(DeepslateTileStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 28566 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 28553 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 28569 {
            return Some(DeepslateTileStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 28576 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 28550 {
            return Some(DeepslateTileStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 28586 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 28575 {
            return Some(DeepslateTileStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 28581 {
            return Some(DeepslateTileStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 28587 {
            return Some(DeepslateTileStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 28593 {
            return Some(DeepslateTileStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 28599 {
            return Some(DeepslateTileStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 28617 {
            return Some(DeepslateTileStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
            });
        }
        if state_id == 28559 {
            return Some(DeepslateTileStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 28564 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 28613 {
            return Some(DeepslateTileStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 28611 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 28578 {
            return Some(DeepslateTileStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
            });
        }
        if state_id == 28556 {
            return Some(DeepslateTileStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 28552 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 28548 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 28579 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 28582 {
            return Some(DeepslateTileStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 28610 {
            return Some(DeepslateTileStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 28624 {
            return Some(DeepslateTileStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 28592 {
            return Some(DeepslateTileStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 28574 {
            return Some(DeepslateTileStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 28588 {
            return Some(DeepslateTileStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 28596 {
            return Some(DeepslateTileStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 28614 {
            return Some(DeepslateTileStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 28583 {
            return Some(DeepslateTileStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 28547 {
            return Some(DeepslateTileStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 28563 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 28567 {
            return Some(DeepslateTileStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 28607 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 28609 {
            return Some(DeepslateTileStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 28619 {
            return Some(DeepslateTileStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 28568 {
            return Some(DeepslateTileStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 28603 {
            return Some(DeepslateTileStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 28621 {
            return Some(DeepslateTileStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}
