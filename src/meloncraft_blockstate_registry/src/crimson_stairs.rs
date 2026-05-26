use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonStairs {
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

impl BlockState for CrimsonStairs {
    fn to_id(&self) -> i32 {
        if self.r#shape == Shape::Straight
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
        {
            return 21144;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
        {
            return 21115;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#shape == Shape::Straight
        {
            return 21124;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
        {
            return 21162;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
        {
            return 21126;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
        {
            return 21145;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::West
        {
            return 21151;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
            && self.r#shape == Shape::Straight
        {
            return 21174;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 21171;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
        {
            return 21172;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 21175;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#shape == Shape::InnerRight
        {
            return 21119;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::North
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == true
        {
            return 21104;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 21155;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#shape == Shape::InnerLeft
        {
            return 21107;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
        {
            return 21169;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 21166;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::South
        {
            return 21136;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterRight
        {
            return 21133;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
        {
            return 21176;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#shape == Shape::OuterRight
        {
            return 21183;
        }
        if self.r#facing == Facing::North
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 21116;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
        {
            return 21127;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::North
        {
            return 21111;
        }
        if self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
        {
            return 21113;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::Straight
        {
            return 21165;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
        {
            return 21121;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#shape == Shape::OuterRight
        {
            return 21132;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::East
        {
            return 21182;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 21108;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
        {
            return 21178;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
        {
            return 21159;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
        {
            return 21150;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#shape == Shape::OuterLeft
        {
            return 21140;
        }
        if self.r#shape == Shape::Straight
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
        {
            return 21125;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
        {
            return 21130;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
        {
            return 21146;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
        {
            return 21138;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 21147;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#shape == Shape::InnerLeft
        {
            return 21106;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
        {
            return 21122;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 21168;
        }
        if self.r#shape == Shape::Straight
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 21154;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#shape == Shape::InnerLeft
        {
            return 21117;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 21142;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::West
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
        {
            return 21161;
        }
        if self.r#shape == Shape::Straight
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 21164;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
        {
            return 21128;
        }
        if self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
        {
            return 21109;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
        {
            return 21139;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::West
        {
            return 21156;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
        {
            return 21157;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
        {
            return 21131;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 21152;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#shape == Shape::Straight
        {
            return 21105;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
        {
            return 21118;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 21135;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#facing == Facing::East
        {
            return 21167;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
        {
            return 21129;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 21163;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
        {
            return 21177;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#shape == Shape::OuterRight
        {
            return 21123;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 21149;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#shape == Shape::Straight
        {
            return 21134;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == false
            && self.r#facing == Facing::West
        {
            return 21153;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 21112;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::North
        {
            return 21120;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 21148;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#shape == Shape::OuterLeft
        {
            return 21160;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
        {
            return 21170;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 21179;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
        {
            return 21173;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == false
        {
            return 21137;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 21141;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterRight
        {
            return 21143;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 21180;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 21110;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::North
        {
            return 21114;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Bottom
        {
            return 21158;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
        {
            return 21181;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21144 {
            return Some(CrimsonStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 21115 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 21124 {
            return Some(CrimsonStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 21162 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 21126 {
            return Some(CrimsonStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 21145 {
            return Some(CrimsonStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 21151 {
            return Some(CrimsonStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 21174 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 21171 {
            return Some(CrimsonStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 21172 {
            return Some(CrimsonStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 21175 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 21119 {
            return Some(CrimsonStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 21104 {
            return Some(CrimsonStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 21155 {
            return Some(CrimsonStairs {
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 21107 {
            return Some(CrimsonStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 21169 {
            return Some(CrimsonStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 21166 {
            return Some(CrimsonStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 21136 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 21133 {
            return Some(CrimsonStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 21176 {
            return Some(CrimsonStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21183 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 21116 {
            return Some(CrimsonStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 21127 {
            return Some(CrimsonStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 21111 {
            return Some(CrimsonStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 21113 {
            return Some(CrimsonStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 21165 {
            return Some(CrimsonStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 21121 {
            return Some(CrimsonStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 21132 {
            return Some(CrimsonStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 21182 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 21108 {
            return Some(CrimsonStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 21178 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 21159 {
            return Some(CrimsonStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 21150 {
            return Some(CrimsonStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 21140 {
            return Some(CrimsonStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 21125 {
            return Some(CrimsonStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 21130 {
            return Some(CrimsonStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 21146 {
            return Some(CrimsonStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 21138 {
            return Some(CrimsonStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21147 {
            return Some(CrimsonStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 21106 {
            return Some(CrimsonStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 21122 {
            return Some(CrimsonStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21168 {
            return Some(CrimsonStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 21154 {
            return Some(CrimsonStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 21117 {
            return Some(CrimsonStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 21142 {
            return Some(CrimsonStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 21161 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 21164 {
            return Some(CrimsonStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 21128 {
            return Some(CrimsonStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 21109 {
            return Some(CrimsonStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 21139 {
            return Some(CrimsonStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21156 {
            return Some(CrimsonStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 21157 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 21131 {
            return Some(CrimsonStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 21152 {
            return Some(CrimsonStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 21105 {
            return Some(CrimsonStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 21118 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 21135 {
            return Some(CrimsonStairs {
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 21167 {
            return Some(CrimsonStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 21129 {
            return Some(CrimsonStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 21163 {
            return Some(CrimsonStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21177 {
            return Some(CrimsonStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 21123 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 21149 {
            return Some(CrimsonStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 21134 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 21153 {
            return Some(CrimsonStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 21112 {
            return Some(CrimsonStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 21120 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 21148 {
            return Some(CrimsonStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 21160 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 21170 {
            return Some(CrimsonStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 21179 {
            return Some(CrimsonStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 21173 {
            return Some(CrimsonStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 21137 {
            return Some(CrimsonStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 21141 {
            return Some(CrimsonStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 21143 {
            return Some(CrimsonStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 21180 {
            return Some(CrimsonStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 21110 {
            return Some(CrimsonStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 21114 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
            });
        }
        if state_id == 21158 {
            return Some(CrimsonStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21181 {
            return Some(CrimsonStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        return None;
    }
}
