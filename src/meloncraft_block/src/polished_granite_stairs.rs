use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedGraniteStairs {
    pub waterlogged: bool,
    pub r#facing: Facing,
    pub r#half: Half,
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

impl BlockState for PolishedGraniteStairs {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::InnerRight { return 15168; }
        if self.r#facing == Facing::North && self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#waterlogged == false { return 15113; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == false { return 15157; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Bottom { return 15129; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::North { return 15107; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft { return 15147; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::North { return 15101; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 15100; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Bottom { return 15125; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#facing == Facing::East && self.r#waterlogged == false { return 15173; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::North { return 15096; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#shape == Shape::OuterRight { return 15133; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::East { return 15167; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 15123; }
        if self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::West && self.r#half == Half::Top { return 15137; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#facing == Facing::East { return 15159; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == true { return 15134; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#waterlogged == false { return 15155; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom { return 15109; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#facing == Facing::South && self.r#waterlogged == true { return 15122; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::Straight && self.r#facing == Facing::West { return 15144; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South && self.r#waterlogged == false { return 15131; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Top { return 15120; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == true { return 15118; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#shape == Shape::Straight { return 15124; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#facing == Facing::North { return 15111; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == false { return 15127; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::South { return 15132; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#facing == Facing::West { return 15140; }
        if self.r#facing == Facing::North && self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#waterlogged == false { return 15103; }
        if self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#half == Half::Top { return 15142; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::InnerRight && self.r#waterlogged == false { return 15169; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::Straight { return 15135; }
        if self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#half == Half::Bottom { return 15126; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#half == Half::Top { return 15162; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Bottom { return 15110; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Top { return 15136; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 15139; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::OuterRight { return 15163; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#half == Half::Top { return 15095; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::OuterRight { return 15102; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#facing == Facing::South { return 15116; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::Straight { return 15115; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Top { return 15141; }
        if self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#facing == Facing::North { return 15097; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::West { return 15148; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 15166; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 15106; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 15153; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#half == Half::Bottom { return 15128; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::Straight { return 15114; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight { return 15108; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 15119; }
        if self.r#facing == Facing::West && self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#half == Half::Top { return 15138; }
        if self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#half == Half::Top { return 15154; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft { return 15117; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::InnerRight { return 15099; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::Straight { return 15104; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#facing == Facing::West { return 15143; }
        if self.r#shape == Shape::Straight && self.r#facing == Facing::West && self.r#waterlogged == false && self.r#half == Half::Bottom { return 15145; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::West { return 15146; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 15112; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::West && self.r#waterlogged == false { return 15151; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 15098; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#facing == Facing::East { return 15156; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == false { return 15121; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 15150; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 15160; }
        if self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#waterlogged == false { return 15165; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 15161; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#facing == Facing::West && self.r#waterlogged == true { return 15152; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::Straight && self.r#facing == Facing::North { return 15094; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom { return 15170; }
        if self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#half == Half::Bottom { return 15130; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 15171; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#facing == Facing::East { return 15158; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#facing == Facing::East { return 15164; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == true && self.r#shape == Shape::OuterRight { return 15172; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::InnerRight { return 15149; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::North { return 15105; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15168 {
            return Some(PolishedGraniteStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15113 {
            return Some(PolishedGraniteStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 15157 {
            return Some(PolishedGraniteStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 15129 {
            return Some(PolishedGraniteStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15107 {
            return Some(PolishedGraniteStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 15147 {
            return Some(PolishedGraniteStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15101 {
            return Some(PolishedGraniteStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 15100 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15125 {
            return Some(PolishedGraniteStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15173 {
            return Some(PolishedGraniteStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 15096 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 15133 {
            return Some(PolishedGraniteStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15167 {
            return Some(PolishedGraniteStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 15123 {
            return Some(PolishedGraniteStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 15137 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 15159 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 15134 {
            return Some(PolishedGraniteStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 15155 {
            return Some(PolishedGraniteStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 15109 {
            return Some(PolishedGraniteStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15122 {
            return Some(PolishedGraniteStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 15144 {
            return Some(PolishedGraniteStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
            });
        }
        if state_id == 15131 {
            return Some(PolishedGraniteStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 15120 {
            return Some(PolishedGraniteStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 15118 {
            return Some(PolishedGraniteStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 15124 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15111 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 15127 {
            return Some(PolishedGraniteStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 15132 {
            return Some(PolishedGraniteStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 15140 {
            return Some(PolishedGraniteStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 15103 {
            return Some(PolishedGraniteStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 15142 {
            return Some(PolishedGraniteStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 15169 {
            return Some(PolishedGraniteStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 15135 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15126 {
            return Some(PolishedGraniteStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15162 {
            return Some(PolishedGraniteStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 15110 {
            return Some(PolishedGraniteStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15136 {
            return Some(PolishedGraniteStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 15139 {
            return Some(PolishedGraniteStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15163 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15095 {
            return Some(PolishedGraniteStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 15102 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15116 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 15115 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15141 {
            return Some(PolishedGraniteStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 15097 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 15148 {
            return Some(PolishedGraniteStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 15166 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15106 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15153 {
            return Some(PolishedGraniteStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15128 {
            return Some(PolishedGraniteStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15114 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15108 {
            return Some(PolishedGraniteStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15119 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15138 {
            return Some(PolishedGraniteStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 15154 {
            return Some(PolishedGraniteStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 15117 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15099 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15104 {
            return Some(PolishedGraniteStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15143 {
            return Some(PolishedGraniteStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 15145 {
            return Some(PolishedGraniteStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15146 {
            return Some(PolishedGraniteStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 15112 {
            return Some(PolishedGraniteStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15151 {
            return Some(PolishedGraniteStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 15098 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15156 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 15121 {
            return Some(PolishedGraniteStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 15150 {
            return Some(PolishedGraniteStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 15160 {
            return Some(PolishedGraniteStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 15165 {
            return Some(PolishedGraniteStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 15161 {
            return Some(PolishedGraniteStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 15152 {
            return Some(PolishedGraniteStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 15094 {
            return Some(PolishedGraniteStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
            });
        }
        if state_id == 15170 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15130 {
            return Some(PolishedGraniteStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15171 {
            return Some(PolishedGraniteStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 15158 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 15164 {
            return Some(PolishedGraniteStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
            });
        }
        if state_id == 15172 {
            return Some(PolishedGraniteStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15149 {
            return Some(PolishedGraniteStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15105 {
            return Some(PolishedGraniteStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

