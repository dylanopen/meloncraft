use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetherBrickStairs {
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

impl BlockState for NetherBrickStairs {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 9205; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerLeft { return 9188; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West { return 9224; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false { return 9241; }
        if block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 9177; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight { return 9175; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East { return 9240; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight { return 9195; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom { return 9182; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight { return 9214; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 9218; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::West && block_state.r#half == Half::Top { return 9210; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterLeft { return 9172; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false { return 9235; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterRight { return 9234; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true { return 9184; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 9222; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true { return 9232; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom { return 9237; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::North && block_state.r#half == Half::Top { return 9168; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 9170; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top && block_state.r#facing == Facing::East { return 9227; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom { return 9225; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft { return 9243; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight { return 9206; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 9209; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#shape == Shape::Straight { return 9216; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true { return 9198; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South { return 9196; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 9194; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 9202; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true { return 9220; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 9221; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 9231; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterLeft { return 9233; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft { return 9183; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft { return 9219; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom { return 9238; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 9178; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 9191; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 9213; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom { return 9179; }
        if block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 9176; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 9223; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterLeft { return 9212; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::East { return 9229; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 9244; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 9197; }
        if block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 9166; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterRight { return 9204; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top { return 9169; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 9200; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerRight { return 9181; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::West { return 9211; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterRight { return 9185; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft { return 9208; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 9174; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 9242; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight { return 9186; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 9215; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft { return 9189; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 9239; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 9199; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight { return 9187; }
        if block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 9236; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::North { return 9171; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 9192; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 9201; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#facing == Facing::East { return 9230; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true { return 9228; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight { return 9190; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top { return 9167; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterLeft { return 9173; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 9245; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::West { return 9207; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerRight { return 9180; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft { return 9193; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom { return 9217; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 9203; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top { return 9226; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9205 {
            return Some(NetherBrickStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 9188 {
            return Some(NetherBrickStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 9224 {
            return Some(NetherBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 9241 {
            return Some(NetherBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 9177 {
            return Some(NetherBrickStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9175 {
            return Some(NetherBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 9240 {
            return Some(NetherBrickStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 9195 {
            return Some(NetherBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 9182 {
            return Some(NetherBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9214 {
            return Some(NetherBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 9218 {
            return Some(NetherBrickStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 9210 {
            return Some(NetherBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 9172 {
            return Some(NetherBrickStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 9235 {
            return Some(NetherBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 9234 {
            return Some(NetherBrickStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 9184 {
            return Some(NetherBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 9222 {
            return Some(NetherBrickStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 9232 {
            return Some(NetherBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 9237 {
            return Some(NetherBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9168 {
            return Some(NetherBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 9170 {
            return Some(NetherBrickStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 9227 {
            return Some(NetherBrickStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 9225 {
            return Some(NetherBrickStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9243 {
            return Some(NetherBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 9206 {
            return Some(NetherBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 9209 {
            return Some(NetherBrickStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 9216 {
            return Some(NetherBrickStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 9198 {
            return Some(NetherBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 9196 {
            return Some(NetherBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 9194 {
            return Some(NetherBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 9202 {
            return Some(NetherBrickStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 9220 {
            return Some(NetherBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 9221 {
            return Some(NetherBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 9231 {
            return Some(NetherBrickStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 9233 {
            return Some(NetherBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 9183 {
            return Some(NetherBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 9219 {
            return Some(NetherBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 9238 {
            return Some(NetherBrickStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9178 {
            return Some(NetherBrickStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 9191 {
            return Some(NetherBrickStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 9213 {
            return Some(NetherBrickStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 9179 {
            return Some(NetherBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9176 {
            return Some(NetherBrickStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 9223 {
            return Some(NetherBrickStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 9212 {
            return Some(NetherBrickStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 9229 {
            return Some(NetherBrickStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 9244 {
            return Some(NetherBrickStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 9197 {
            return Some(NetherBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 9166 {
            return Some(NetherBrickStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 9204 {
            return Some(NetherBrickStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 9169 {
            return Some(NetherBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 9200 {
            return Some(NetherBrickStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 9181 {
            return Some(NetherBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 9211 {
            return Some(NetherBrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 9185 {
            return Some(NetherBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 9208 {
            return Some(NetherBrickStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 9174 {
            return Some(NetherBrickStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 9242 {
            return Some(NetherBrickStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 9186 {
            return Some(NetherBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 9215 {
            return Some(NetherBrickStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 9189 {
            return Some(NetherBrickStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 9239 {
            return Some(NetherBrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9199 {
            return Some(NetherBrickStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 9187 {
            return Some(NetherBrickStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 9236 {
            return Some(NetherBrickStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9171 {
            return Some(NetherBrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
            });
        }
        if state_id == 9192 {
            return Some(NetherBrickStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 9201 {
            return Some(NetherBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 9230 {
            return Some(NetherBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 9228 {
            return Some(NetherBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 9190 {
            return Some(NetherBrickStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 9167 {
            return Some(NetherBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 9173 {
            return Some(NetherBrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 9245 {
            return Some(NetherBrickStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 9207 {
            return Some(NetherBrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
            });
        }
        if state_id == 9180 {
            return Some(NetherBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 9193 {
            return Some(NetherBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 9217 {
            return Some(NetherBrickStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 9203 {
            return Some(NetherBrickStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 9226 {
            return Some(NetherBrickStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        return None;
    }
}

