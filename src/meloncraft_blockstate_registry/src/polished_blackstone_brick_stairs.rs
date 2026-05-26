use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedBlackstoneBrickStairs {
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

impl BlockState for PolishedBlackstoneBrickStairs {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
        {
            return 22124;
        }
        if self.r#shape == Shape::Straight
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 22101;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
        {
            return 22053;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#shape == Shape::OuterRight
        {
            return 22059;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
        {
            return 22074;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Top
        {
            return 22090;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 22076;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
        {
            return 22094;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 22107;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 22118;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
        {
            return 22068;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#shape == Shape::Straight
        {
            return 22051;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 22069;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#shape == Shape::InnerLeft
        {
            return 22083;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
        {
            return 22079;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
        {
            return 22100;
        }
        if self.r#facing == Facing::North
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 22058;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#shape == Shape::OuterRight
        {
            return 22119;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
        {
            return 22084;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 22109;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::West
        {
            return 22105;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
        {
            return 22072;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 22062;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
        {
            return 22066;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#shape == Shape::OuterLeft
        {
            return 22087;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Top
        {
            return 22057;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 22111;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
        {
            return 22112;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == false
        {
            return 22063;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
        {
            return 22121;
        }
        if self.r#shape == Shape::Straight
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 22050;
        }
        if self.r#shape == Shape::Straight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
        {
            return 22060;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
        {
            return 22127;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::West
        {
            return 22108;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#shape == Shape::Straight
        {
            return 22120;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
        {
            return 22128;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 22110;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#shape == Shape::Straight
        {
            return 22080;
        }
        if self.r#facing == Facing::North
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 22064;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 22088;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 22098;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
        {
            return 22055;
        }
        if self.r#shape == Shape::Straight
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
        {
            return 22071;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 22089;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
        {
            return 22070;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
        {
            return 22125;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::East
        {
            return 22123;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
        {
            return 22082;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
        {
            return 22093;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 22061;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
        {
            return 22095;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
        {
            return 22102;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::East
        {
            return 22113;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 22077;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
        {
            return 22104;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
        {
            return 22052;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 22116;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
        {
            return 22117;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 22085;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 22115;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 22126;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
        {
            return 22114;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Bottom
        {
            return 22122;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::North
        {
            return 22067;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
        {
            return 22129;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
        {
            return 22056;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::West
        {
            return 22097;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::South
        {
            return 22078;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
        {
            return 22091;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
        {
            return 22103;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
        {
            return 22075;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
        {
            return 22092;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 22081;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
        {
            return 22073;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
        {
            return 22106;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
        {
            return 22065;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
        {
            return 22096;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 22099;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::South
        {
            return 22086;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
        {
            return 22054;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22124 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 22101 {
            return Some(PolishedBlackstoneBrickStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 22053 {
            return Some(PolishedBlackstoneBrickStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 22059 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 22074 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 22090 {
            return Some(PolishedBlackstoneBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 22076 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 22094 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 22107 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 22118 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 22068 {
            return Some(PolishedBlackstoneBrickStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 22051 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 22069 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 22083 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 22079 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 22100 {
            return Some(PolishedBlackstoneBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 22058 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 22119 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 22084 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 22109 {
            return Some(PolishedBlackstoneBrickStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 22105 {
            return Some(PolishedBlackstoneBrickStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 22072 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 22062 {
            return Some(PolishedBlackstoneBrickStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 22066 {
            return Some(PolishedBlackstoneBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 22087 {
            return Some(PolishedBlackstoneBrickStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 22057 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 22111 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 22112 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 22063 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 22121 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 22050 {
            return Some(PolishedBlackstoneBrickStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 22060 {
            return Some(PolishedBlackstoneBrickStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 22127 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 22108 {
            return Some(PolishedBlackstoneBrickStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 22120 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 22128 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 22110 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 22080 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 22064 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 22088 {
            return Some(PolishedBlackstoneBrickStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 22098 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 22055 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 22071 {
            return Some(PolishedBlackstoneBrickStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 22089 {
            return Some(PolishedBlackstoneBrickStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 22070 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 22125 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 22123 {
            return Some(PolishedBlackstoneBrickStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 22082 {
            return Some(PolishedBlackstoneBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 22093 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 22061 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 22095 {
            return Some(PolishedBlackstoneBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 22102 {
            return Some(PolishedBlackstoneBrickStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 22113 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 22077 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 22104 {
            return Some(PolishedBlackstoneBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 22052 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 22116 {
            return Some(PolishedBlackstoneBrickStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 22117 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 22085 {
            return Some(PolishedBlackstoneBrickStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 22115 {
            return Some(PolishedBlackstoneBrickStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 22126 {
            return Some(PolishedBlackstoneBrickStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 22114 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 22122 {
            return Some(PolishedBlackstoneBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 22067 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 22129 {
            return Some(PolishedBlackstoneBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 22056 {
            return Some(PolishedBlackstoneBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 22097 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 22078 {
            return Some(PolishedBlackstoneBrickStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
            });
        }
        if state_id == 22091 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 22103 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 22075 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 22092 {
            return Some(PolishedBlackstoneBrickStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 22081 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 22073 {
            return Some(PolishedBlackstoneBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 22106 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 22065 {
            return Some(PolishedBlackstoneBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 22096 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 22099 {
            return Some(PolishedBlackstoneBrickStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 22086 {
            return Some(PolishedBlackstoneBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 22054 {
            return Some(PolishedBlackstoneBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        return None;
    }
}
