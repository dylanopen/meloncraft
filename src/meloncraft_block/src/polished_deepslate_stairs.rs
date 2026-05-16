use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedDeepslateStairs {
    pub r#half: Half,
    pub r#shape: Shape,
    pub r#facing: Facing,
    pub waterlogged: bool,
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

impl BlockState for PolishedDeepslateStairs {
    fn to_id(&self) -> i32 {
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#facing == Facing::West { return 28192; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::Straight { return 28136; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 28140; }
        if self.r#facing == Facing::North && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#waterlogged == false { return 28151; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#half == Half::Top { return 28164; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft { return 28202; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::OuterRight { return 28175; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == true { return 28206; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 28186; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 28160; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft { return 28163; }
        if self.r#facing == Facing::North && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#waterlogged == true { return 28150; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top { return 28198; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 28183; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 28188; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft { return 28172; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight { return 28214; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#facing == Facing::East && self.r#waterlogged == false { return 28211; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#facing == Facing::West { return 28194; }
        if self.r#waterlogged == false && self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#facing == Facing::North { return 28141; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Top { return 28144; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == true { return 28190; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#facing == Facing::East { return 28197; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Bottom { return 28171; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 28191; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#facing == Facing::North && self.r#waterlogged == true { return 28146; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#facing == Facing::West && self.r#waterlogged == true { return 28184; }
        if self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#facing == Facing::West && self.r#waterlogged == false { return 28185; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::East { return 28203; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft { return 28162; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 28155; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 28153; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft { return 28178; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 28209; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#facing == Facing::North { return 28137; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East { return 28208; }
        if self.r#facing == Facing::South && self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#half == Half::Bottom { return 28166; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::East { return 28213; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#facing == Facing::South { return 28167; }
        if self.r#facing == Facing::East && self.r#shape == Shape::OuterRight && self.r#half == Half::Bottom && self.r#waterlogged == false { return 28215; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#shape == Shape::Straight && self.r#half == Half::Bottom { return 28147; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 28207; }
        if self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#half == Half::Bottom { return 28212; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 28139; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::North { return 28152; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::South { return 28168; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North { return 28143; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Top { return 28199; }
        if self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::West && self.r#half == Half::Top { return 28182; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == false { return 28195; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::InnerRight && self.r#waterlogged == false { return 28161; }
        if self.r#facing == Facing::East && self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#half == Half::Top { return 28200; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::North { return 28149; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Top { return 28177; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Bottom { return 28170; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft { return 28159; }
        if self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#waterlogged == true { return 28196; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#facing == Facing::South { return 28165; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 28189; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#shape == Shape::OuterRight { return 28145; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#half == Half::Top { return 28205; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Top { return 28180; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#facing == Facing::West { return 28193; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::South { return 28158; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft { return 28173; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::OuterRight { return 28154; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == true { return 28210; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Bottom { return 28187; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 28142; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 28176; }
        if self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom && self.r#waterlogged == true { return 28148; }
        if self.r#facing == Facing::South && self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#half == Half::Top { return 28156; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 28169; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 28174; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::West { return 28179; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft { return 28138; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Top { return 28157; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 28181; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 28201; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#facing == Facing::East { return 28204; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 28192 {
            return Some(PolishedDeepslateStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 28136 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 28140 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 28151 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 28164 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 28202 {
            return Some(PolishedDeepslateStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 28175 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 28206 {
            return Some(PolishedDeepslateStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 28186 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 28160 {
            return Some(PolishedDeepslateStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 28163 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 28150 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 28198 {
            return Some(PolishedDeepslateStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 28183 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 28188 {
            return Some(PolishedDeepslateStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 28172 {
            return Some(PolishedDeepslateStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 28214 {
            return Some(PolishedDeepslateStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 28211 {
            return Some(PolishedDeepslateStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 28194 {
            return Some(PolishedDeepslateStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 28141 {
            return Some(PolishedDeepslateStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 28144 {
            return Some(PolishedDeepslateStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 28190 {
            return Some(PolishedDeepslateStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 28197 {
            return Some(PolishedDeepslateStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
            });
        }
        if state_id == 28171 {
            return Some(PolishedDeepslateStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 28191 {
            return Some(PolishedDeepslateStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 28146 {
            return Some(PolishedDeepslateStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 28184 {
            return Some(PolishedDeepslateStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 28185 {
            return Some(PolishedDeepslateStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 28203 {
            return Some(PolishedDeepslateStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 28162 {
            return Some(PolishedDeepslateStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 28155 {
            return Some(PolishedDeepslateStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 28153 {
            return Some(PolishedDeepslateStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 28178 {
            return Some(PolishedDeepslateStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 28209 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 28137 {
            return Some(PolishedDeepslateStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
            });
        }
        if state_id == 28208 {
            return Some(PolishedDeepslateStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 28166 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 28213 {
            return Some(PolishedDeepslateStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 28167 {
            return Some(PolishedDeepslateStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
            });
        }
        if state_id == 28215 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 28147 {
            return Some(PolishedDeepslateStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 28207 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 28212 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 28139 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 28152 {
            return Some(PolishedDeepslateStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 28168 {
            return Some(PolishedDeepslateStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 28143 {
            return Some(PolishedDeepslateStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 28199 {
            return Some(PolishedDeepslateStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 28182 {
            return Some(PolishedDeepslateStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 28195 {
            return Some(PolishedDeepslateStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 28161 {
            return Some(PolishedDeepslateStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 28200 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 28149 {
            return Some(PolishedDeepslateStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 28177 {
            return Some(PolishedDeepslateStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 28170 {
            return Some(PolishedDeepslateStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 28159 {
            return Some(PolishedDeepslateStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 28196 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 28165 {
            return Some(PolishedDeepslateStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
            });
        }
        if state_id == 28189 {
            return Some(PolishedDeepslateStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 28145 {
            return Some(PolishedDeepslateStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 28205 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 28180 {
            return Some(PolishedDeepslateStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 28193 {
            return Some(PolishedDeepslateStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 28158 {
            return Some(PolishedDeepslateStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 28173 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 28154 {
            return Some(PolishedDeepslateStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 28210 {
            return Some(PolishedDeepslateStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 28187 {
            return Some(PolishedDeepslateStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 28142 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 28176 {
            return Some(PolishedDeepslateStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 28148 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 28156 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 28169 {
            return Some(PolishedDeepslateStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 28174 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 28179 {
            return Some(PolishedDeepslateStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 28138 {
            return Some(PolishedDeepslateStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 28157 {
            return Some(PolishedDeepslateStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 28181 {
            return Some(PolishedDeepslateStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 28201 {
            return Some(PolishedDeepslateStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 28204 {
            return Some(PolishedDeepslateStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

