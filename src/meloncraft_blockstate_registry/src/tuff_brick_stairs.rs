use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuffBrickStairs {
    pub r#shape: Shape,
    pub r#half: Half,
    pub r#facing: Facing,
    pub waterlogged: bool,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

impl BlockState for TuffBrickStairs {
    fn to_id(&self) -> i32 {
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::North
        {
            return 24092;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
        {
            return 24143;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
        {
            return 24085;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 24087;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Bottom
        {
            return 24132;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
        {
            return 24142;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::North
        {
            return 24084;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
        {
            return 24104;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 24128;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 24133;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::West
        {
            return 24125;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::West
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == true
        {
            return 24130;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#shape == Shape::Straight
        {
            return 24150;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::South
        {
            return 24107;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 24145;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Top
        {
            return 24100;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
        {
            return 24096;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
        {
            return 24129;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
        {
            return 24154;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
        {
            return 24086;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Top
        {
            return 24081;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == false
        {
            return 24103;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
        {
            return 24091;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
        {
            return 24114;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#shape == Shape::OuterLeft
        {
            return 24136;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#half == Half::Top
        {
            return 24148;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 24095;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
        {
            return 24083;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
        {
            return 24124;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 24126;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#shape == Shape::Straight
        {
            return 24080;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
        {
            return 24122;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::South
        {
            return 24109;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
        {
            return 24146;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 24153;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == true
        {
            return 24090;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterRight
        {
            return 24139;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
        {
            return 24099;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::South
        {
            return 24111;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
        {
            return 24098;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
        {
            return 24097;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#shape == Shape::InnerLeft
        {
            return 24113;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 24119;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::West
        {
            return 24138;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
        {
            return 24088;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
        {
            return 24094;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
        {
            return 24159;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 24123;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
        {
            return 24106;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 24155;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
        {
            return 24147;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
        {
            return 24149;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#shape == Shape::Straight
        {
            return 24110;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#shape == Shape::OuterLeft
        {
            return 24157;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#shape == Shape::OuterRight
        {
            return 24089;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 24105;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
        {
            return 24118;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
        {
            return 24112;
        }
        if self.r#shape == Shape::Straight
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::West
        {
            return 24131;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 24121;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
        {
            return 24116;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::East
        {
            return 24140;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Bottom
        {
            return 24156;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::West
            && self.r#shape == Shape::InnerRight
        {
            return 24134;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
        {
            return 24137;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
            && self.r#shape == Shape::OuterLeft
        {
            return 24127;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
        {
            return 24158;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 24141;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 24117;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#shape == Shape::Straight
        {
            return 24120;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
        {
            return 24144;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::Straight
        {
            return 24101;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
        {
            return 24102;
        }
        if self.r#facing == Facing::South
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Top
        {
            return 24108;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#shape == Shape::InnerRight
        {
            return 24115;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
        {
            return 24093;
        }
        if self.r#shape == Shape::Straight
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 24151;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
        {
            return 24082;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 24152;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::West
        {
            return 24135;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 24092 {
            return Some(TuffBrickStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 24143 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 24085 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 24087 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 24132 {
            return Some(TuffBrickStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 24142 {
            return Some(TuffBrickStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 24084 {
            return Some(TuffBrickStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 24104 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 24128 {
            return Some(TuffBrickStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 24133 {
            return Some(TuffBrickStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 24125 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 24130 {
            return Some(TuffBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 24150 {
            return Some(TuffBrickStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 24107 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 24145 {
            return Some(TuffBrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 24100 {
            return Some(TuffBrickStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 24096 {
            return Some(TuffBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 24129 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 24154 {
            return Some(TuffBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 24086 {
            return Some(TuffBrickStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 24081 {
            return Some(TuffBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 24103 {
            return Some(TuffBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 24091 {
            return Some(TuffBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 24114 {
            return Some(TuffBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 24136 {
            return Some(TuffBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 24148 {
            return Some(TuffBrickStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 24095 {
            return Some(TuffBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 24083 {
            return Some(TuffBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 24124 {
            return Some(TuffBrickStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 24126 {
            return Some(TuffBrickStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 24080 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 24122 {
            return Some(TuffBrickStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 24109 {
            return Some(TuffBrickStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
            });
        }
        if state_id == 24146 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 24153 {
            return Some(TuffBrickStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 24090 {
            return Some(TuffBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 24139 {
            return Some(TuffBrickStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 24099 {
            return Some(TuffBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 24111 {
            return Some(TuffBrickStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
            });
        }
        if state_id == 24098 {
            return Some(TuffBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 24097 {
            return Some(TuffBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 24113 {
            return Some(TuffBrickStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 24119 {
            return Some(TuffBrickStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 24138 {
            return Some(TuffBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 24088 {
            return Some(TuffBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 24094 {
            return Some(TuffBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 24159 {
            return Some(TuffBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 24123 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 24106 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 24155 {
            return Some(TuffBrickStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 24147 {
            return Some(TuffBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 24149 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 24110 {
            return Some(TuffBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 24157 {
            return Some(TuffBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 24089 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 24105 {
            return Some(TuffBrickStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 24118 {
            return Some(TuffBrickStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 24112 {
            return Some(TuffBrickStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 24131 {
            return Some(TuffBrickStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 24121 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 24116 {
            return Some(TuffBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 24140 {
            return Some(TuffBrickStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
            });
        }
        if state_id == 24156 {
            return Some(TuffBrickStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 24134 {
            return Some(TuffBrickStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 24137 {
            return Some(TuffBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 24127 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 24158 {
            return Some(TuffBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 24141 {
            return Some(TuffBrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 24117 {
            return Some(TuffBrickStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 24120 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 24144 {
            return Some(TuffBrickStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 24101 {
            return Some(TuffBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 24102 {
            return Some(TuffBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 24108 {
            return Some(TuffBrickStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 24115 {
            return Some(TuffBrickStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 24093 {
            return Some(TuffBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 24151 {
            return Some(TuffBrickStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 24082 {
            return Some(TuffBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 24152 {
            return Some(TuffBrickStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 24135 {
            return Some(TuffBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
            });
        }
        return None;
    }
}
