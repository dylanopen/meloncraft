use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuffStairs {
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

impl BlockState for TuffStairs {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 23293; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom { return 23268; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 23312; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#shape == Shape::Straight { return 23317; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true { return 23323; }
        if block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 23266; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 23289; }
        if block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 23327; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight { return 23325; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 23319; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight { return 23272; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true { return 23333; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom { return 23308; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::North { return 23273; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true { return 23271; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false { return 23296; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 23282; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 23299; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft { return 23314; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 23309; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::East && block_state.r#half == Half::Top { return 23318; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerRight { return 23331; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::East { return 23332; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight { return 23265; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 23334; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 23259; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 23321; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top { return 23264; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 23290; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 23277; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft { return 23300; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false { return 23260; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft { return 23324; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft { return 23274; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::North { return 23258; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 23284; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North { return 23275; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft { return 23280; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 23283; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight { return 23301; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight { return 23291; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top { return 23298; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterRight { return 23315; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterLeft { return 23304; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerRight { return 23322; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::South { return 23278; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 23306; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight { return 23305; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom { return 23307; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight { return 23285; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom { return 23310; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#shape == Shape::Straight { return 23257; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 23269; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 23281; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::South { return 23288; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 23286; }
        if block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top { return 23297; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::West { return 23302; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::West { return 23313; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom { return 23335; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft { return 23294; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom { return 23336; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 23316; }
        if block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North { return 23267; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true { return 23303; }
        if block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 23287; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 23329; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false { return 23270; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::East { return 23326; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom { return 23311; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerRight { return 23261; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom { return 23295; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 23292; }
        if block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom { return 23328; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom { return 23330; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::North && block_state.r#half == Half::Top { return 23263; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight { return 23262; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::South { return 23279; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 23276; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerLeft { return 23320; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23293 {
            return Some(TuffStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 23268 {
            return Some(TuffStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 23312 {
            return Some(TuffStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 23317 {
            return Some(TuffStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 23323 {
            return Some(TuffStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 23266 {
            return Some(TuffStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 23289 {
            return Some(TuffStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 23327 {
            return Some(TuffStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 23325 {
            return Some(TuffStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 23319 {
            return Some(TuffStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 23272 {
            return Some(TuffStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 23333 {
            return Some(TuffStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 23308 {
            return Some(TuffStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 23273 {
            return Some(TuffStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 23271 {
            return Some(TuffStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 23296 {
            return Some(TuffStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 23282 {
            return Some(TuffStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 23299 {
            return Some(TuffStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 23314 {
            return Some(TuffStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 23309 {
            return Some(TuffStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 23318 {
            return Some(TuffStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 23331 {
            return Some(TuffStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 23332 {
            return Some(TuffStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 23265 {
            return Some(TuffStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 23334 {
            return Some(TuffStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 23259 {
            return Some(TuffStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 23321 {
            return Some(TuffStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 23264 {
            return Some(TuffStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 23290 {
            return Some(TuffStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 23277 {
            return Some(TuffStairs {
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 23300 {
            return Some(TuffStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 23260 {
            return Some(TuffStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 23324 {
            return Some(TuffStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 23274 {
            return Some(TuffStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 23258 {
            return Some(TuffStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
            });
        }
        if state_id == 23284 {
            return Some(TuffStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 23275 {
            return Some(TuffStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 23280 {
            return Some(TuffStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 23283 {
            return Some(TuffStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 23301 {
            return Some(TuffStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 23291 {
            return Some(TuffStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 23298 {
            return Some(TuffStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 23315 {
            return Some(TuffStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 23304 {
            return Some(TuffStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 23322 {
            return Some(TuffStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 23278 {
            return Some(TuffStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
            });
        }
        if state_id == 23306 {
            return Some(TuffStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 23305 {
            return Some(TuffStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 23307 {
            return Some(TuffStairs {
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 23285 {
            return Some(TuffStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 23310 {
            return Some(TuffStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 23257 {
            return Some(TuffStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 23269 {
            return Some(TuffStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 23281 {
            return Some(TuffStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 23288 {
            return Some(TuffStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
            });
        }
        if state_id == 23286 {
            return Some(TuffStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 23297 {
            return Some(TuffStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 23302 {
            return Some(TuffStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 23313 {
            return Some(TuffStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 23335 {
            return Some(TuffStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 23294 {
            return Some(TuffStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 23336 {
            return Some(TuffStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 23316 {
            return Some(TuffStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 23267 {
            return Some(TuffStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 23303 {
            return Some(TuffStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 23287 {
            return Some(TuffStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 23329 {
            return Some(TuffStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 23270 {
            return Some(TuffStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 23326 {
            return Some(TuffStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 23311 {
            return Some(TuffStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 23261 {
            return Some(TuffStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 23295 {
            return Some(TuffStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 23292 {
            return Some(TuffStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 23328 {
            return Some(TuffStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 23330 {
            return Some(TuffStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 23263 {
            return Some(TuffStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 23262 {
            return Some(TuffStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 23279 {
            return Some(TuffStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 23276 {
            return Some(TuffStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 23320 {
            return Some(TuffStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
            });
        }
        return None;
    }
}

