use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedStairs {
    pub r#half: Half,
    pub waterlogged: bool,
    pub r#shape: Shape,
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

impl BlockState for WarpedStairs {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::North && self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#half == Half::Top { return 21184; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight { return 21198; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::West && self.r#half == Half::Top { return 21230; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#half == Half::Top { return 21191; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == true { return 21220; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Bottom { return 21235; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::OuterRight { return 21252; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight { return 21223; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == true { return 21228; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#facing == Facing::East { return 21256; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Bottom { return 21261; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::West && self.r#waterlogged == false { return 21237; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 21207; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft { return 21236; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::East { return 21249; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Bottom { return 21219; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Top { return 21188; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == false { return 21217; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 21240; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 21201; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 21255; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 21197; }
        if self.r#waterlogged == true && self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#facing == Facing::South { return 21214; }
        if self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#half == Half::Top { return 21245; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::East { return 21251; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#facing == Facing::East { return 21246; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#shape == Shape::InnerRight { return 21258; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::OuterRight { return 21213; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Top { return 21190; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 21211; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 21216; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft { return 21221; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 21238; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft { return 21187; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#shape == Shape::OuterRight { return 21192; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom { return 21200; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == false { return 21263; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::Straight { return 21194; }
        if self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::North { return 21186; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::West { return 21241; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom { return 21242; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::InnerRight { return 21218; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#half == Half::Top { return 21185; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::InnerRight { return 21189; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#facing == Facing::East { return 21262; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::OuterRight { return 21193; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::Straight { return 21204; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::North { return 21195; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 21226; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#half == Half::Bottom { return 21254; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Top { return 21247; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Bottom { return 21259; }
        if self.r#facing == Facing::East && self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#half == Half::Top { return 21253; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 21196; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#waterlogged == false { return 21199; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::South { return 21212; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Bottom { return 21222; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom { return 21257; }
        if self.r#facing == Facing::South && self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#half == Half::Top { return 21209; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::South && self.r#waterlogged == true { return 21206; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 21202; }
        if self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#half == Half::Top { return 21233; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#waterlogged == false { return 21205; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#facing == Facing::West && self.r#waterlogged == true { return 21224; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 21250; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#facing == Facing::South { return 21215; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == false { return 21229; }
        if self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#facing == Facing::West && self.r#half == Half::Top { return 21225; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::Straight { return 21234; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Bottom { return 21203; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#waterlogged == false { return 21239; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight { return 21243; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == true { return 21248; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 21208; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#facing == Facing::West && self.r#waterlogged == true { return 21232; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Top { return 21244; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 21260; }
        if self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::West { return 21227; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#facing == Facing::West && self.r#waterlogged == false { return 21231; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#half == Half::Top { return 21210; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21184 {
            return Some(WarpedStairs {
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 21198 {
            return Some(WarpedStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 21230 {
            return Some(WarpedStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 21191 {
            return Some(WarpedStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 21220 {
            return Some(WarpedStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 21235 {
            return Some(WarpedStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21252 {
            return Some(WarpedStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 21223 {
            return Some(WarpedStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 21228 {
            return Some(WarpedStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 21256 {
            return Some(WarpedStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 21261 {
            return Some(WarpedStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21237 {
            return Some(WarpedStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 21207 {
            return Some(WarpedStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 21236 {
            return Some(WarpedStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 21249 {
            return Some(WarpedStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 21219 {
            return Some(WarpedStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21188 {
            return Some(WarpedStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 21217 {
            return Some(WarpedStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 21240 {
            return Some(WarpedStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 21201 {
            return Some(WarpedStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 21255 {
            return Some(WarpedStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 21197 {
            return Some(WarpedStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 21214 {
            return Some(WarpedStairs {
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 21245 {
            return Some(WarpedStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 21251 {
            return Some(WarpedStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 21246 {
            return Some(WarpedStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 21258 {
            return Some(WarpedStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 21213 {
            return Some(WarpedStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 21190 {
            return Some(WarpedStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 21211 {
            return Some(WarpedStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 21216 {
            return Some(WarpedStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 21221 {
            return Some(WarpedStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 21238 {
            return Some(WarpedStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 21187 {
            return Some(WarpedStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 21192 {
            return Some(WarpedStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 21200 {
            return Some(WarpedStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21263 {
            return Some(WarpedStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 21194 {
            return Some(WarpedStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 21186 {
            return Some(WarpedStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 21241 {
            return Some(WarpedStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 21242 {
            return Some(WarpedStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21218 {
            return Some(WarpedStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 21185 {
            return Some(WarpedStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 21189 {
            return Some(WarpedStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 21262 {
            return Some(WarpedStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 21193 {
            return Some(WarpedStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 21204 {
            return Some(WarpedStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 21195 {
            return Some(WarpedStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 21226 {
            return Some(WarpedStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 21254 {
            return Some(WarpedStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21247 {
            return Some(WarpedStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 21259 {
            return Some(WarpedStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21253 {
            return Some(WarpedStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 21196 {
            return Some(WarpedStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 21199 {
            return Some(WarpedStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 21212 {
            return Some(WarpedStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 21222 {
            return Some(WarpedStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21257 {
            return Some(WarpedStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21209 {
            return Some(WarpedStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 21206 {
            return Some(WarpedStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 21202 {
            return Some(WarpedStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21233 {
            return Some(WarpedStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 21205 {
            return Some(WarpedStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 21224 {
            return Some(WarpedStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 21250 {
            return Some(WarpedStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 21215 {
            return Some(WarpedStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
            });
        }
        if state_id == 21229 {
            return Some(WarpedStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 21225 {
            return Some(WarpedStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 21234 {
            return Some(WarpedStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 21203 {
            return Some(WarpedStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21239 {
            return Some(WarpedStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 21243 {
            return Some(WarpedStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 21248 {
            return Some(WarpedStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 21208 {
            return Some(WarpedStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 21232 {
            return Some(WarpedStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 21244 {
            return Some(WarpedStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 21260 {
            return Some(WarpedStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 21227 {
            return Some(WarpedStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 21231 {
            return Some(WarpedStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 21210 {
            return Some(WarpedStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        return None;
    }
}

