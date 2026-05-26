use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatheredCutCopperStairs {
    pub r#facing: Facing,
    pub r#half: Half,
    pub waterlogged: bool,
    pub r#shape: Shape,
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

impl BlockState for WeatheredCutCopperStairs {
    fn to_id(&self) -> i32 {
        if self.r#half == Half::Top
            && self.r#facing == Facing::North
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
        {
            return 25208;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
        {
            return 25277;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 25278;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
        {
            return 25255;
        }
        if self.r#waterlogged == true
            && self.r#facing == Facing::South
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
        {
            return 25243;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::South
        {
            return 25229;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#shape == Shape::OuterRight
        {
            return 25274;
        }
        if self.r#facing == Facing::North
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 25212;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
        {
            return 25242;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 25244;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
        {
            return 25276;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 25234;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Bottom
        {
            return 25264;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterLeft
        {
            return 25271;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::North
        {
            return 25220;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 25235;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
        {
            return 25249;
        }
        if self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
        {
            return 25258;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::South
            && self.r#waterlogged == true
        {
            return 25241;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
        {
            return 25247;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
        {
            return 25263;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 25265;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 25251;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
        {
            return 25273;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::Straight
        {
            return 25236;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
        {
            return 25259;
        }
        if self.r#facing == Facing::North
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 25222;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 25232;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
        {
            return 25230;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 25233;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::East
        {
            return 25279;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#waterlogged == false
        {
            return 25248;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#shape == Shape::Straight
        {
            return 25205;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == false
            && self.r#half == Half::Bottom
        {
            return 25240;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#shape == Shape::InnerRight
        {
            return 25209;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::South
        {
            return 25226;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
        {
            return 25270;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::East
            && self.r#half == Half::Bottom
        {
            return 25284;
        }
        if self.r#half == Half::Top
            && self.r#facing == Facing::West
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
        {
            return 25252;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Top
            && self.r#facing == Facing::North
            && self.r#shape == Shape::OuterRight
        {
            return 25213;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::West
            && self.r#half == Half::Bottom
            && self.r#waterlogged == false
        {
            return 25262;
        }
        if self.r#waterlogged == true
            && self.r#shape == Shape::InnerLeft
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
        {
            return 25217;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
        {
            return 25218;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::North
            && self.r#half == Half::Bottom
        {
            return 25216;
        }
        if self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::South
        {
            return 25237;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 25207;
        }
        if self.r#facing == Facing::South
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 25225;
        }
        if self.r#shape == Shape::Straight
            && self.r#facing == Facing::West
            && self.r#waterlogged == true
            && self.r#half == Half::Top
        {
            return 25245;
        }
        if self.r#facing == Facing::East
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
        {
            return 25268;
        }
        if self.r#facing == Facing::North
            && self.r#shape == Shape::OuterLeft
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 25211;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterRight
            && self.r#facing == Facing::North
        {
            return 25224;
        }
        if self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
        {
            return 25227;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 25266;
        }
        if self.r#facing == Facing::West
            && self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#shape == Shape::OuterRight
        {
            return 25254;
        }
        if self.r#shape == Shape::OuterRight
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::North
        {
            return 25223;
        }
        if self.r#facing == Facing::North
            && self.r#shape == Shape::InnerRight
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 25219;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::Straight
            && self.r#half == Half::Top
            && self.r#facing == Facing::West
        {
            return 25246;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
        {
            return 25269;
        }
        if self.r#facing == Facing::East
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 25281;
        }
        if self.r#shape == Shape::InnerLeft
            && self.r#facing == Facing::South
            && self.r#waterlogged == false
            && self.r#half == Half::Top
        {
            return 25228;
        }
        if self.r#half == Half::Top
            && self.r#shape == Shape::InnerLeft
            && self.r#waterlogged == true
            && self.r#facing == Facing::East
        {
            return 25267;
        }
        if self.r#facing == Facing::East
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#waterlogged == false
        {
            return 25282;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::West
        {
            return 25250;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#facing == Facing::West
            && self.r#shape == Shape::InnerLeft
        {
            return 25257;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::North
            && self.r#half == Half::Top
            && self.r#shape == Shape::InnerRight
        {
            return 25210;
        }
        if self.r#half == Half::Bottom
            && self.r#waterlogged == true
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::North
        {
            return 25215;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#shape == Shape::Straight
        {
            return 25275;
        }
        if self.r#waterlogged == false
            && self.r#half == Half::Top
            && self.r#facing == Facing::East
            && self.r#shape == Shape::OuterLeft
        {
            return 25272;
        }
        if self.r#facing == Facing::West
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 25253;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::West
            && self.r#shape == Shape::Straight
            && self.r#waterlogged == false
        {
            return 25256;
        }
        if self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#waterlogged == true
            && self.r#shape == Shape::OuterRight
        {
            return 25283;
        }
        if self.r#waterlogged == false
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::InnerLeft
        {
            return 25238;
        }
        if self.r#facing == Facing::North
            && self.r#shape == Shape::OuterRight
            && self.r#half == Half::Top
            && self.r#waterlogged == false
        {
            return 25214;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::North
            && self.r#waterlogged == true
            && self.r#half == Half::Bottom
        {
            return 25221;
        }
        if self.r#waterlogged == true
            && self.r#half == Half::Bottom
            && self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::West
        {
            return 25261;
        }
        if self.r#half == Half::Top
            && self.r#waterlogged == false
            && self.r#shape == Shape::Straight
            && self.r#facing == Facing::North
        {
            return 25206;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#facing == Facing::South
            && self.r#half == Half::Bottom
            && self.r#waterlogged == true
        {
            return 25239;
        }
        if self.r#shape == Shape::InnerRight
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::East
            && self.r#waterlogged == false
        {
            return 25280;
        }
        if self.r#waterlogged == false
            && self.r#shape == Shape::InnerRight
            && self.r#half == Half::Bottom
            && self.r#facing == Facing::West
        {
            return 25260;
        }
        if self.r#shape == Shape::OuterLeft
            && self.r#facing == Facing::South
            && self.r#half == Half::Top
            && self.r#waterlogged == true
        {
            return 25231;
        }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25208 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25277 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 25278 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25255 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 25243 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25229 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 25274 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25212 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25242 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25244 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 25276 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25234 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 25264 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25271 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25220 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
            });
        }
        if state_id == 25235 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 25249 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25258 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25241 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 25247 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 25263 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 25265 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 25251 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 25273 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25236 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25259 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25222 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25232 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 25230 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25233 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 25279 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 25248 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 25205 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25240 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25209 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25226 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
            });
        }
        if state_id == 25270 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25284 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25252 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 25213 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25262 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 25217 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 25218 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25216 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25237 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 25207 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 25225 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 25245 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 25268 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25211 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 25224 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
            });
        }
        if state_id == 25227 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 25266 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 25254 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25223 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 25219 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25246 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 25269 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 25281 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25228 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 25267 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 25282 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 25250 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 25257 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25210 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25215 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
            });
        }
        if state_id == 25275 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25272 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25253 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 25256 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 25283 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25238 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25214 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25221 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25261 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 25206 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
            });
        }
        if state_id == 25239 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25280 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 25260 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 25231 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        return None;
    }
}
