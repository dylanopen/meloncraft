use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatheredCutCopperStairs {
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

impl BlockState for WeatheredCutCopperStairs {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 25219; }
        if block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 25213; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 25250; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true { return 25243; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight { return 25226; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top { return 25249; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West { return 25257; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 25232; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 25265; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft { return 25217; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerLeft { return 25208; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 25230; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 25271; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top { return 25211; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 25253; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#facing == Facing::North { return 25210; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::South { return 25228; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::South { return 25231; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 25275; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 25263; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 25240; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 25262; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom { return 25282; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 25283; }
        if block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 25206; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 25272; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterRight { return 25224; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 25252; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 25264; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::South { return 25241; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 25225; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top { return 25248; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom { return 25259; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 25261; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 25238; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 25216; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true { return 25223; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 25278; }
        if block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 25276; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight { return 25234; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 25280; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true { return 25207; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#shape == Shape::Straight { return 25245; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight { return 25244; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 25256; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 25235; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top { return 25247; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 25270; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true { return 25205; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft { return 25242; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight { return 25279; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 25212; }
        if block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::West { return 25246; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false { return 25284; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterRight { return 25214; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true { return 25239; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top { return 25233; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft { return 25237; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom { return 25215; }
        if block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 25209; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#shape == Shape::Straight { return 25266; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false { return 25258; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::North { return 25220; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft { return 25222; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false { return 25254; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East { return 25281; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 25221; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 25255; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true { return 25251; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight { return 25260; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::East { return 25269; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerRight { return 25229; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 25268; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false { return 25274; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom { return 25277; }
        if block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 25218; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 25227; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true { return 25273; }
        if block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South { return 25236; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top { return 25267; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25219 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25213 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 25250 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 25243 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 25226 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25249 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 25257 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#facing: Facing::West,
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
        if state_id == 25265 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 25217 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25208 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25230 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 25271 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 25211 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 25253 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 25210 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 25228 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 25231 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 25275 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Bottom,
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
        if state_id == 25240 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 25262 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25282 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25283 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 25206 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25272 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25224 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25252 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 25264 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 25241 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 25225 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 25248 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 25259 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25261 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25238 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 25216 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 25223 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 25278 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 25276 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 25234 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25280 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25207 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 25245 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25244 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25256 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::West,
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
        if state_id == 25247 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 25270 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 25205 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 25242 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25279 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25212 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25246 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 25284 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 25214 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25239 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 25233 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 25237 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25215 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25209 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 25266 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25258 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
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
        if state_id == 25222 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25254 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 25281 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 25221 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 25255 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25251 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 25260 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25269 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 25229 {
            return Some(WeatheredCutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25268 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 25274 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 25277 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25218 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 25227 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 25273 {
            return Some(WeatheredCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 25236 {
            return Some(WeatheredCutCopperStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 25267 {
            return Some(WeatheredCutCopperStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        return None;
    }
}

