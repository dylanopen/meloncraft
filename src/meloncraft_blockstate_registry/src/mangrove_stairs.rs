use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangroveStairs {
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

impl BlockState for MangroveStairs {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
        {
            return 12119;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
        {
            return 12164;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
        {
            return 12113;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
        {
            return 12123;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
        {
            return 12148;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
        {
            return 12142;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
        {
            return 12159;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
        {
            return 12139;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#shape == Shape::OuterLeft
        {
            return 12166;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Top
        {
            return 12099;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Top
        {
            return 12096;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::North
        {
            return 12098;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
        {
            return 12101;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
        {
            return 12152;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
        {
            return 12162;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
        {
            return 12117;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::North
            && self.r#shape == Shape::InnerRight
        {
            return 12094;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
        {
            return 12102;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::West
        {
            return 12132;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
        {
            return 12121;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 12150;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::North
            && self.r#shape == Shape::InnerLeft
        {
            return 12092;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 12107;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 12160;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Bottom
        {
            return 12165;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
        {
            return 12143;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
        {
            return 12127;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
        {
            return 12095;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
        {
            return 12106;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
        {
            return 12161;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 12122;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 12137;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
        {
            return 12138;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
        {
            return 12141;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::North
        {
            return 12105;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
        {
            return 12145;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
        {
            return 12154;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 12128;
        }
        if self.r#facing == Facing::North
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 12097;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 12112;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
        {
            return 12090;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
        {
            return 12114;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 12118;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Bottom
        {
            return 12104;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
        {
            return 12115;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#shape == Shape::Straight
        {
            return 12110;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 12124;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 12108;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#shape == Shape::Straight
        {
            return 12111;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 12130;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
        {
            return 12134;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Top
        {
            return 12136;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
        {
            return 12149;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
        {
            return 12109;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#shape == Shape::InnerLeft
        {
            return 12153;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::East
        {
            return 12156;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 12157;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 12093;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
        {
            return 12135;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
        {
            return 12140;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
        {
            return 12167;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 12144;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
        {
            return 12158;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 12133;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 12168;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
        {
            return 12147;
        }
        if self.r#shape == Shape::Straight
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 12151;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 12116;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
        {
            return 12103;
        }
        if self.r#shape == Shape::Straight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 12120;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::South
        {
            return 12125;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == false
        {
            return 12163;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::East
            && self.r#half == Half::Top
        {
            return 12155;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::North
        {
            return 12091;
        }
        if self.r#facing == Facing::North
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 12100;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
        {
            return 12129;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
        {
            return 12131;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::East
        {
            return 12169;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
        {
            return 12126;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#shape == Shape::OuterLeft
        {
            return 12146;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12119 {
            return Some(MangroveStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 12164 {
            return Some(MangroveStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12113 {
            return Some(MangroveStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12123 {
            return Some(MangroveStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 12148 {
            return Some(MangroveStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12142 {
            return Some(MangroveStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12159 {
            return Some(MangroveStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 12139 {
            return Some(MangroveStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 12166 {
            return Some(MangroveStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 12099 {
            return Some(MangroveStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 12096 {
            return Some(MangroveStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 12098 {
            return Some(MangroveStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 12101 {
            return Some(MangroveStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 12152 {
            return Some(MangroveStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12162 {
            return Some(MangroveStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12117 {
            return Some(MangroveStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 12094 {
            return Some(MangroveStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 12102 {
            return Some(MangroveStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12132 {
            return Some(MangroveStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 12121 {
            return Some(MangroveStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12150 {
            return Some(MangroveStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 12092 {
            return Some(MangroveStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12107 {
            return Some(MangroveStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 12160 {
            return Some(MangroveStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12165 {
            return Some(MangroveStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12143 {
            return Some(MangroveStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12127 {
            return Some(MangroveStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12095 {
            return Some(MangroveStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 12106 {
            return Some(MangroveStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 12161 {
            return Some(MangroveStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12122 {
            return Some(MangroveStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 12137 {
            return Some(MangroveStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 12138 {
            return Some(MangroveStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 12141 {
            return Some(MangroveStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12105 {
            return Some(MangroveStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
            });
        }
        if state_id == 12145 {
            return Some(MangroveStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 12154 {
            return Some(MangroveStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 12128 {
            return Some(MangroveStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 12097 {
            return Some(MangroveStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 12112 {
            return Some(MangroveStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 12090 {
            return Some(MangroveStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 12114 {
            return Some(MangroveStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 12118 {
            return Some(MangroveStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 12104 {
            return Some(MangroveStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12115 {
            return Some(MangroveStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 12110 {
            return Some(MangroveStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12124 {
            return Some(MangroveStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12108 {
            return Some(MangroveStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 12111 {
            return Some(MangroveStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12130 {
            return Some(MangroveStairs {
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 12134 {
            return Some(MangroveStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 12136 {
            return Some(MangroveStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 12149 {
            return Some(MangroveStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 12109 {
            return Some(MangroveStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 12153 {
            return Some(MangroveStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12156 {
            return Some(MangroveStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 12157 {
            return Some(MangroveStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 12093 {
            return Some(MangroveStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 12135 {
            return Some(MangroveStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 12140 {
            return Some(MangroveStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 12167 {
            return Some(MangroveStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 12144 {
            return Some(MangroveStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 12158 {
            return Some(MangroveStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 12133 {
            return Some(MangroveStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 12168 {
            return Some(MangroveStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 12147 {
            return Some(MangroveStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 12151 {
            return Some(MangroveStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 12116 {
            return Some(MangroveStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 12103 {
            return Some(MangroveStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 12120 {
            return Some(MangroveStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 12125 {
            return Some(MangroveStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
            });
        }
        if state_id == 12163 {
            return Some(MangroveStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 12155 {
            return Some(MangroveStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 12091 {
            return Some(MangroveStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
            });
        }
        if state_id == 12100 {
            return Some(MangroveStairs {
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 12129 {
            return Some(MangroveStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 12131 {
            return Some(MangroveStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 12169 {
            return Some(MangroveStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 12126 {
            return Some(MangroveStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 12146 {
            return Some(MangroveStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        return None;
    }
}
