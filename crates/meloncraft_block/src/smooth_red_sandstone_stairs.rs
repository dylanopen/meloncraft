use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmoothRedSandstoneStairs {
    pub r#shape: Shape,
    pub r#facing: Facing,
    pub waterlogged: bool,
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

impl BlockState for SmoothRedSandstoneStairs {
    fn to_id(self) -> i32 {
        if block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 15183; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom { return 15249; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom { return 15230; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false { return 15209; }
        if block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 15195; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::East { return 15240; }
        if block_state.r#facing == Facing::North && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 15174; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top { return 15203; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false { return 15199; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 15193; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 15248; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 15217; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Top { return 15236; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterLeft { return 15210; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false { return 15241; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight { return 15239; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight { return 15252; }
        if block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 15214; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight { return 15194; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::North { return 15192; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::North { return 15175; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true { return 15182; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterLeft { return 15180; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::South { return 15201; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 15227; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight { return 15204; }
        if block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 15184; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom { return 15206; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 15185; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft { return 15207; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight { return 15215; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterLeft { return 15221; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight { return 15224; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true { return 15222; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true { return 15202; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom { return 15246; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom { return 15208; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false { return 15177; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false { return 15189; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top && block_state.r#facing == Facing::North { return 15181; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom { return 15233; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false { return 15243; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 15251; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 15253; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 15191; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 15234; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 15187; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#shape == Shape::Straight { return 15225; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 15188; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East { return 15247; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom { return 15211; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 15218; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterRight { return 15223; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::East && block_state.r#half == Half::Top { return 15235; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft { return 15216; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom { return 15228; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight { return 15232; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 15219; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterLeft { return 15220; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false { return 15229; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 15179; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top { return 15237; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::South { return 15196; }
        if block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 15190; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom { return 15250; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 15238; }
        if block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 15176; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::South { return 15200; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 15231; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true { return 15242; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::North { return 15186; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false { return 15205; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom { return 15212; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerLeft { return 15197; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#facing == Facing::North { return 15178; }
        if block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 15244; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 15226; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom { return 15213; }
        if block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East { return 15245; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 15198; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15183 {
            return Some(SmoothRedSandstoneStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 15249 {
            return Some(SmoothRedSandstoneStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15230 {
            return Some(SmoothRedSandstoneStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15209 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 15195 {
            return Some(SmoothRedSandstoneStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 15240 {
            return Some(SmoothRedSandstoneStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 15174 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 15203 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 15199 {
            return Some(SmoothRedSandstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 15193 {
            return Some(SmoothRedSandstoneStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 15248 {
            return Some(SmoothRedSandstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 15217 {
            return Some(SmoothRedSandstoneStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 15236 {
            return Some(SmoothRedSandstoneStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 15210 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15241 {
            return Some(SmoothRedSandstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 15239 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15252 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15214 {
            return Some(SmoothRedSandstoneStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 15194 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15192 {
            return Some(SmoothRedSandstoneStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
            });
        }
        if state_id == 15175 {
            return Some(SmoothRedSandstoneStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
            });
        }
        if state_id == 15182 {
            return Some(SmoothRedSandstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15180 {
            return Some(SmoothRedSandstoneStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15201 {
            return Some(SmoothRedSandstoneStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 15227 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15204 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15184 {
            return Some(SmoothRedSandstoneStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15206 {
            return Some(SmoothRedSandstoneStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15185 {
            return Some(SmoothRedSandstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 15207 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15215 {
            return Some(SmoothRedSandstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15221 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15224 {
            return Some(SmoothRedSandstoneStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15222 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15202 {
            return Some(SmoothRedSandstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15246 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15208 {
            return Some(SmoothRedSandstoneStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15177 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 15189 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 15181 {
            return Some(SmoothRedSandstoneStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 15233 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15243 {
            return Some(SmoothRedSandstoneStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 15251 {
            return Some(SmoothRedSandstoneStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 15253 {
            return Some(SmoothRedSandstoneStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 15191 {
            return Some(SmoothRedSandstoneStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 15234 {
            return Some(SmoothRedSandstoneStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 15187 {
            return Some(SmoothRedSandstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 15225 {
            return Some(SmoothRedSandstoneStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15188 {
            return Some(SmoothRedSandstoneStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 15247 {
            return Some(SmoothRedSandstoneStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 15211 {
            return Some(SmoothRedSandstoneStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15218 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 15223 {
            return Some(SmoothRedSandstoneStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15235 {
            return Some(SmoothRedSandstoneStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 15216 {
            return Some(SmoothRedSandstoneStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15228 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15232 {
            return Some(SmoothRedSandstoneStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15219 {
            return Some(SmoothRedSandstoneStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 15220 {
            return Some(SmoothRedSandstoneStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15229 {
            return Some(SmoothRedSandstoneStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 15179 {
            return Some(SmoothRedSandstoneStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 15237 {
            return Some(SmoothRedSandstoneStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 15196 {
            return Some(SmoothRedSandstoneStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 15190 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15250 {
            return Some(SmoothRedSandstoneStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15238 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 15176 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 15200 {
            return Some(SmoothRedSandstoneStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 15231 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 15242 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15186 {
            return Some(SmoothRedSandstoneStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 15205 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 15212 {
            return Some(SmoothRedSandstoneStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15197 {
            return Some(SmoothRedSandstoneStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15178 {
            return Some(SmoothRedSandstoneStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 15244 {
            return Some(SmoothRedSandstoneStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 15226 {
            return Some(SmoothRedSandstoneStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 15213 {
            return Some(SmoothRedSandstoneStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15245 {
            return Some(SmoothRedSandstoneStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 15198 {
            return Some(SmoothRedSandstoneStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

