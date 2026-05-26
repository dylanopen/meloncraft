use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MossyCobblestoneStairs {
    pub r#facing: Facing,
    pub r#shape: Shape,
    pub waterlogged: bool,
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

impl BlockState for MossyCobblestoneStairs {
    fn to_id(&self) -> i32 {
        if self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 15469;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
        {
            return 15470;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 15443;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
        {
            return 15437;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
        {
            return 15450;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
        {
            return 15479;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
        {
            return 15442;
        }
        if self.r#facing == Facing::North
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 15422;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
        {
            return 15489;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#shape == Shape::InnerLeft
        {
            return 15447;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterRight
        {
            return 15483;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 15491;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
        {
            return 15438;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
        {
            return 15449;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
            && self.r#facing == Facing::North
        {
            return 15416;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
        {
            return 15433;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
        {
            return 15417;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
        {
            return 15476;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Bottom
        {
            return 15424;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#shape == Shape::OuterRight
        {
            return 15462;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::West
        {
            return 15468;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
        {
            return 15418;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 15440;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::South
        {
            return 15441;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::West
            && self.r#shape == Shape::Straight
        {
            return 15465;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
        {
            return 15457;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 15451;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
        {
            return 15493;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == true
        {
            return 15464;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
        {
            return 15429;
        }
        if self.r#shape == Shape::Straight
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
        {
            return 15484;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
        {
            return 15486;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
        {
            return 15420;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Top
        {
            return 15474;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
        {
            return 15490;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
        {
            return 15445;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::Straight
        {
            return 15454;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
        {
            return 15426;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 15428;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Bottom
        {
            return 15444;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::West
        {
            return 15472;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#shape == Shape::OuterRight
        {
            return 15463;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#shape == Shape::Straight
        {
            return 15435;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 15478;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
        {
            return 15477;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 15436;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
        {
            return 15487;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
        {
            return 15492;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
        {
            return 15431;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 15482;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
        {
            return 15460;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 15448;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 15475;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
        {
            return 15456;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 15439;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 15488;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
        {
            return 15453;
        }
        if self.r#facing == Facing::North
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 15414;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterRight
        {
            return 15423;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
        {
            return 15452;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
        {
            return 15481;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
        {
            return 15427;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 15467;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::North
        {
            return 15425;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::Straight
        {
            return 15485;
        }
        if self.r#shape == Shape::Straight
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
        {
            return 15455;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
        {
            return 15461;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
        {
            return 15430;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
        {
            return 15458;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 15419;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::East
        {
            return 15480;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::North
        {
            return 15421;
        }
        if self.r#shape == Shape::Straight
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 15434;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 15473;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
        {
            return 15446;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
        {
            return 15432;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::West
        {
            return 15459;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 15466;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::West
        {
            return 15471;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::North
        {
            return 15415;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15469 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15470 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15443 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 15437 {
            return Some(MossyCobblestoneStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15450 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 15479 {
            return Some(MossyCobblestoneStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 15442 {
            return Some(MossyCobblestoneStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15422 {
            return Some(MossyCobblestoneStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 15489 {
            return Some(MossyCobblestoneStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15447 {
            return Some(MossyCobblestoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15483 {
            return Some(MossyCobblestoneStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15491 {
            return Some(MossyCobblestoneStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15438 {
            return Some(MossyCobblestoneStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15449 {
            return Some(MossyCobblestoneStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15416 {
            return Some(MossyCobblestoneStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 15433 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 15417 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 15476 {
            return Some(MossyCobblestoneStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 15424 {
            return Some(MossyCobblestoneStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15462 {
            return Some(MossyCobblestoneStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15468 {
            return Some(MossyCobblestoneStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 15418 {
            return Some(MossyCobblestoneStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 15440 {
            return Some(MossyCobblestoneStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 15441 {
            return Some(MossyCobblestoneStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 15465 {
            return Some(MossyCobblestoneStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15457 {
            return Some(MossyCobblestoneStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 15451 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15493 {
            return Some(MossyCobblestoneStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15464 {
            return Some(MossyCobblestoneStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 15429 {
            return Some(MossyCobblestoneStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 15484 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15486 {
            return Some(MossyCobblestoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 15420 {
            return Some(MossyCobblestoneStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 15474 {
            return Some(MossyCobblestoneStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 15490 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 15445 {
            return Some(MossyCobblestoneStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 15454 {
            return Some(MossyCobblestoneStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15426 {
            return Some(MossyCobblestoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 15428 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 15444 {
            return Some(MossyCobblestoneStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15472 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 15463 {
            return Some(MossyCobblestoneStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15435 {
            return Some(MossyCobblestoneStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15478 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 15477 {
            return Some(MossyCobblestoneStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 15436 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 15487 {
            return Some(MossyCobblestoneStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15492 {
            return Some(MossyCobblestoneStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15431 {
            return Some(MossyCobblestoneStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 15482 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 15460 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 15448 {
            return Some(MossyCobblestoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 15475 {
            return Some(MossyCobblestoneStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 15456 {
            return Some(MossyCobblestoneStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 15439 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 15488 {
            return Some(MossyCobblestoneStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15453 {
            return Some(MossyCobblestoneStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 15414 {
            return Some(MossyCobblestoneStairs {
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 15423 {
            return Some(MossyCobblestoneStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15452 {
            return Some(MossyCobblestoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15481 {
            return Some(MossyCobblestoneStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15427 {
            return Some(MossyCobblestoneStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15467 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15425 {
            return Some(MossyCobblestoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
            });
        }
        if state_id == 15485 {
            return Some(MossyCobblestoneStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15455 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 15461 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 15430 {
            return Some(MossyCobblestoneStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15458 {
            return Some(MossyCobblestoneStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 15419 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 15480 {
            return Some(MossyCobblestoneStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 15421 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 15434 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 15473 {
            return Some(MossyCobblestoneStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15446 {
            return Some(MossyCobblestoneStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15432 {
            return Some(MossyCobblestoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15459 {
            return Some(MossyCobblestoneStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 15466 {
            return Some(MossyCobblestoneStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15471 {
            return Some(MossyCobblestoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 15415 {
            return Some(MossyCobblestoneStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}
