use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DioriteStairs {
    pub r#half: Half,
    pub r#shape: Shape,
    pub waterlogged: bool,
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

impl BlockState for DioriteStairs {
    fn to_id(self) -> i32 {
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::North { return 16138; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 16160; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Top { return 16140; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top && block_state.r#facing == Facing::East { return 16195; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 16157; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top { return 16177; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 16149; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 16173; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 16187; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true { return 16164; }
        if block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 16147; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 16145; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South { return 16166; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#shape == Shape::Straight { return 16144; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 16186; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::South { return 16154; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom { return 16168; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom { return 16204; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom { return 16152; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true { return 16208; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::East { return 16211; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 16150; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top { return 16203; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterLeft { return 16210; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight { return 16153; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerLeft { return 16196; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft { return 16200; }
        if block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 16165; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North { return 16148; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West { return 16185; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::West { return 16174; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 16180; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 16146; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false { return 16167; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight { return 16172; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 16161; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight { return 16189; }
        if block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 16205; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterRight { return 16213; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::North { return 16136; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 16197; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false { return 16137; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::West { return 16181; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight { return 16162; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top { return 16135; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true { return 16158; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom { return 16190; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::North { return 16139; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterLeft { return 16141; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 16171; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight { return 16183; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top { return 16201; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 16209; }
        if block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 16194; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 16142; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true { return 16212; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 16188; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight { return 16198; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 16192; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 16143; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false { return 16169; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#facing == Facing::South { return 16159; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterLeft { return 16151; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerRight { return 16199; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterLeft { return 16191; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight { return 16202; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false { return 16163; }
        if block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 16134; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 16156; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 16176; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight { return 16178; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 16179; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterLeft { return 16170; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom { return 16206; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 16207; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 16193; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight { return 16184; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 16182; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 16175; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false { return 16155; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16138 {
            return Some(DioriteStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
            });
        }
        if state_id == 16160 {
            return Some(DioriteStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 16140 {
            return Some(DioriteStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 16195 {
            return Some(DioriteStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 16157 {
            return Some(DioriteStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 16177 {
            return Some(DioriteStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 16149 {
            return Some(DioriteStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 16173 {
            return Some(DioriteStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 16187 {
            return Some(DioriteStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 16164 {
            return Some(DioriteStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 16147 {
            return Some(DioriteStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 16145 {
            return Some(DioriteStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 16166 {
            return Some(DioriteStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 16144 {
            return Some(DioriteStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 16186 {
            return Some(DioriteStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 16154 {
            return Some(DioriteStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
            });
        }
        if state_id == 16168 {
            return Some(DioriteStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 16204 {
            return Some(DioriteStairs {
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 16152 {
            return Some(DioriteStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 16208 {
            return Some(DioriteStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 16211 {
            return Some(DioriteStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 16150 {
            return Some(DioriteStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 16203 {
            return Some(DioriteStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 16210 {
            return Some(DioriteStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 16153 {
            return Some(DioriteStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 16196 {
            return Some(DioriteStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 16200 {
            return Some(DioriteStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 16165 {
            return Some(DioriteStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 16148 {
            return Some(DioriteStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 16185 {
            return Some(DioriteStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 16174 {
            return Some(DioriteStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
            });
        }
        if state_id == 16180 {
            return Some(DioriteStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 16146 {
            return Some(DioriteStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 16167 {
            return Some(DioriteStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 16172 {
            return Some(DioriteStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 16161 {
            return Some(DioriteStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 16189 {
            return Some(DioriteStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 16205 {
            return Some(DioriteStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 16213 {
            return Some(DioriteStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 16136 {
            return Some(DioriteStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 16197 {
            return Some(DioriteStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 16137 {
            return Some(DioriteStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 16181 {
            return Some(DioriteStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 16162 {
            return Some(DioriteStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 16135 {
            return Some(DioriteStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 16158 {
            return Some(DioriteStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 16190 {
            return Some(DioriteStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 16139 {
            return Some(DioriteStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 16141 {
            return Some(DioriteStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 16171 {
            return Some(DioriteStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 16183 {
            return Some(DioriteStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 16201 {
            return Some(DioriteStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 16209 {
            return Some(DioriteStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 16194 {
            return Some(DioriteStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 16142 {
            return Some(DioriteStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 16212 {
            return Some(DioriteStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 16188 {
            return Some(DioriteStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 16198 {
            return Some(DioriteStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 16192 {
            return Some(DioriteStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 16143 {
            return Some(DioriteStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 16169 {
            return Some(DioriteStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 16159 {
            return Some(DioriteStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 16151 {
            return Some(DioriteStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 16199 {
            return Some(DioriteStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 16191 {
            return Some(DioriteStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 16202 {
            return Some(DioriteStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 16163 {
            return Some(DioriteStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 16134 {
            return Some(DioriteStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 16156 {
            return Some(DioriteStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 16176 {
            return Some(DioriteStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 16178 {
            return Some(DioriteStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 16179 {
            return Some(DioriteStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 16170 {
            return Some(DioriteStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 16206 {
            return Some(DioriteStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 16207 {
            return Some(DioriteStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 16193 {
            return Some(DioriteStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 16184 {
            return Some(DioriteStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 16182 {
            return Some(DioriteStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 16175 {
            return Some(DioriteStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 16155 {
            return Some(DioriteStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

