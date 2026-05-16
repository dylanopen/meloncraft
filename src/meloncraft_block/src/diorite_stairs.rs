use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DioriteStairs {
    pub waterlogged: bool,
    pub r#half: Half,
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

impl BlockState for DioriteStairs {
    fn to_id(&self) -> i32 {
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::West { return 16192; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::North && self.r#half == Half::Top { return 16136; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Top { return 16200; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == true { return 16162; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft { return 16210; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 16160; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::Straight && self.r#half == Half::Bottom { return 16164; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == false { return 16173; }
        if self.r#facing == Facing::South && self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#waterlogged == false { return 16165; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#facing == Facing::West { return 16178; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::InnerRight { return 16188; }
        if self.r#shape == Shape::InnerRight && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Bottom { return 16209; }
        if self.r#shape == Shape::InnerRight && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Top { return 16158; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::South { return 16167; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 16211; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Top { return 16140; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#facing == Facing::North && self.r#half == Half::Top { return 16143; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::InnerRight { return 16148; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 16138; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == true { return 16176; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top { return 16156; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#facing == Facing::North { return 16145; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#shape == Shape::Straight { return 16154; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::South { return 16157; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East && self.r#waterlogged == false { return 16207; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft { return 16196; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::OuterRight { return 16142; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 16182; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::OuterRight { return 16153; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 16152; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::InnerRight { return 16168; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::InnerRight { return 16179; }
        if self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#half == Half::Top { return 16177; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 16193; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::Straight { return 16194; }
        if self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#facing == Facing::South && self.r#half == Half::Top { return 16155; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 16195; }
        if self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 16150; }
        if self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#half == Half::Bottom { return 16187; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 16208; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true { return 16166; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::Straight && self.r#facing == Facing::North { return 16144; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 16212; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#shape == Shape::Straight && self.r#half == Half::Top { return 16134; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::OuterRight { return 16213; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 16206; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == false { return 16181; }
        if self.r#waterlogged == true && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 16146; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East { return 16201; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#waterlogged == true { return 16174; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::OuterRight { return 16183; }
        if self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#half == Half::Bottom { return 16204; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::OuterRight { return 16203; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#shape == Shape::Straight { return 16184; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::West { return 16186; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::East { return 16198; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 16161; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Bottom { return 16190; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#shape == Shape::InnerRight && self.r#half == Half::Top { return 16159; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 16172; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::InnerRight && self.r#waterlogged == false { return 16149; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft { return 16141; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::InnerRight { return 16139; }
        if self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::OuterRight { return 16202; }
        if self.r#facing == Facing::South && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#waterlogged == true { return 16170; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::Straight { return 16205; }
        if self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom { return 16191; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == false { return 16163; }
        if self.r#facing == Facing::West && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#waterlogged == false { return 16189; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 16175; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false { return 16197; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Bottom { return 16151; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::InnerRight { return 16199; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::West { return 16185; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::North { return 16137; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::OuterLeft { return 16180; }
        if self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 16147; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 16171; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::Straight && self.r#facing == Facing::North { return 16135; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#waterlogged == false { return 16169; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16192 {
            return Some(DioriteStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 16136 {
            return Some(DioriteStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 16200 {
            return Some(DioriteStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 16162 {
            return Some(DioriteStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
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
        if state_id == 16160 {
            return Some(DioriteStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 16164 {
            return Some(DioriteStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 16173 {
            return Some(DioriteStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 16165 {
            return Some(DioriteStairs {
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 16178 {
            return Some(DioriteStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 16188 {
            return Some(DioriteStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 16209 {
            return Some(DioriteStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 16158 {
            return Some(DioriteStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 16167 {
            return Some(DioriteStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 16211 {
            return Some(DioriteStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
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
        if state_id == 16143 {
            return Some(DioriteStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 16148 {
            return Some(DioriteStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 16138 {
            return Some(DioriteStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 16176 {
            return Some(DioriteStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 16156 {
            return Some(DioriteStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
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
        if state_id == 16154 {
            return Some(DioriteStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 16157 {
            return Some(DioriteStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 16207 {
            return Some(DioriteStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 16196 {
            return Some(DioriteStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 16142 {
            return Some(DioriteStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 16182 {
            return Some(DioriteStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 16153 {
            return Some(DioriteStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 16152 {
            return Some(DioriteStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 16168 {
            return Some(DioriteStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 16179 {
            return Some(DioriteStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 16177 {
            return Some(DioriteStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 16193 {
            return Some(DioriteStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 16194 {
            return Some(DioriteStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 16155 {
            return Some(DioriteStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 16195 {
            return Some(DioriteStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 16150 {
            return Some(DioriteStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 16187 {
            return Some(DioriteStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
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
        if state_id == 16166 {
            return Some(DioriteStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 16144 {
            return Some(DioriteStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
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
        if state_id == 16134 {
            return Some(DioriteStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 16213 {
            return Some(DioriteStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 16206 {
            return Some(DioriteStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 16181 {
            return Some(DioriteStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 16146 {
            return Some(DioriteStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 16201 {
            return Some(DioriteStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 16174 {
            return Some(DioriteStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 16183 {
            return Some(DioriteStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 16204 {
            return Some(DioriteStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 16203 {
            return Some(DioriteStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 16184 {
            return Some(DioriteStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 16186 {
            return Some(DioriteStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 16198 {
            return Some(DioriteStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 16161 {
            return Some(DioriteStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 16190 {
            return Some(DioriteStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 16159 {
            return Some(DioriteStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 16172 {
            return Some(DioriteStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 16149 {
            return Some(DioriteStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 16141 {
            return Some(DioriteStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 16139 {
            return Some(DioriteStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 16202 {
            return Some(DioriteStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 16170 {
            return Some(DioriteStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 16205 {
            return Some(DioriteStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 16191 {
            return Some(DioriteStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 16163 {
            return Some(DioriteStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 16189 {
            return Some(DioriteStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 16175 {
            return Some(DioriteStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 16197 {
            return Some(DioriteStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 16151 {
            return Some(DioriteStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 16199 {
            return Some(DioriteStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 16185 {
            return Some(DioriteStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 16137 {
            return Some(DioriteStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 16180 {
            return Some(DioriteStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 16147 {
            return Some(DioriteStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 16171 {
            return Some(DioriteStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 16135 {
            return Some(DioriteStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
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
        return None;
    }
}

