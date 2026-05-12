use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MossyStoneBrickStairs {
    pub r#half: Half,
    pub r#facing: Facing,
    pub r#shape: Shape,
    pub waterlogged: bool,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Half {
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Facing {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Straight,
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight,
}

impl BlockState for MossyStoneBrickStairs {
    fn to_id(self) -> i32 {
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::West { return 15302; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false { return 15331; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight { return 15333; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight { return 15294; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft { return 15300; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 15327; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 15329; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 15279; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom { return 15309; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight { return 15305; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 15323; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::South && block_state.r#waterlogged == false { return 15285; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West { return 15307; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 15324; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom { return 15328; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top { return 15295; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 15276; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft { return 15257; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom { return 15312; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterRight { return 15313; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterLeft { return 15261; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight { return 15263; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true { return 15288; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top { return 15256; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::South { return 15282; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true { return 15308; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#shape == Shape::Straight { return 15255; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false { return 15271; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West { return 15310; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 15301; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerRight { return 15268; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true { return 15270; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 15286; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North { return 15273; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 15318; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 15316; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 15292; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 15262; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false { return 15275; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 15291; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::West { return 15306; }
        if block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Top { return 15315; }
        if block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 15264; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top { return 15280; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::South && block_state.r#half == Half::Top { return 15281; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#shape == Shape::Straight { return 15265; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 15317; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 15319; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::South && block_state.r#half == Half::Top { return 15274; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 15326; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 15260; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 15303; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight { return 15284; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top { return 15259; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 15272; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterRight { return 15283; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West { return 15311; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 15314; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterLeft { return 15330; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight { return 15269; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft { return 15267; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom { return 15266; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerRight { return 15278; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::West && block_state.r#half == Half::Top { return 15296; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false { return 15293; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 15298; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top { return 15254; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false { return 15287; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 15277; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::East { return 15332; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 15289; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::East { return 15320; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top && block_state.r#facing == Facing::East { return 15321; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight { return 15299; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight { return 15304; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom { return 15325; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 15258; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 15322; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South { return 15290; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false { return 15297; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15302 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 15331 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 15333 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15294 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15300 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15327 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 15329 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 15279 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 15309 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15305 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15323 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 15285 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 15307 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 15324 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 15328 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15295 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 15276 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 15257 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15312 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15313 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15261 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15263 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15288 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15256 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 15282 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 15308 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15255 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15271 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 15310 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 15301 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 15268 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15270 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 15286 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 15273 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 15318 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 15316 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 15292 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 15262 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 15275 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 15291 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 15306 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 15315 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 15264 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 15280 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 15281 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 15265 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15317 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 15319 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 15274 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 15326 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 15260 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 15303 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 15284 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15259 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 15272 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 15283 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15311 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 15314 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 15330 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15269 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15267 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15266 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15278 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15296 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 15293 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 15298 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 15254 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 15287 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 15277 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 15332 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 15289 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15320 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 15321 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 15299 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15304 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15325 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15258 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 15322 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 15290 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 15297 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

