use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MossyStoneBrickStairs {
    pub r#shape: Shape,
    pub r#facing: Facing,
    pub r#half: Half,
    pub waterlogged: bool,
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

impl BlockState for MossyStoneBrickStairs {
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::InnerLeft { return 15276; }
        if self.r#shape == Shape::OuterLeft && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#waterlogged == true { return 15290; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::InnerLeft { return 15257; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#waterlogged == false && self.r#shape == Shape::Straight { return 15295; }
        if self.r#facing == Facing::West && self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#half == Half::Top { return 15299; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Top { return 15320; }
        if self.r#waterlogged == true && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::North && self.r#half == Half::Bottom { return 15266; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::OuterLeft { return 15271; }
        if self.r#facing == Facing::West && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#half == Half::Bottom { return 15306; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 15278; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#shape == Shape::Straight { return 15274; }
        if self.r#shape == Shape::OuterRight && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#waterlogged == true { return 15332; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft { return 15317; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::InnerRight { return 15319; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#facing == Facing::North && self.r#waterlogged == true { return 15258; }
        if self.r#facing == Facing::North && self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#waterlogged == true { return 15264; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#shape == Shape::Straight { return 15305; }
        if self.r#shape == Shape::OuterRight && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::West { return 15312; }
        if self.r#half == Half::Top && self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#facing == Facing::West { return 15296; }
        if self.r#facing == Facing::East && self.r#shape == Shape::OuterRight && self.r#half == Half::Top && self.r#waterlogged == true { return 15322; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::InnerRight && self.r#facing == Facing::East { return 15318; }
        if self.r#facing == Facing::North && self.r#shape == Shape::Straight && self.r#half == Half::Top && self.r#waterlogged == true { return 15254; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#shape == Shape::InnerRight { return 15269; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft { return 15311; }
        if self.r#shape == Shape::InnerLeft && self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == true { return 15286; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#half == Half::Top && self.r#facing == Facing::West { return 15297; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 15321; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#waterlogged == false { return 15331; }
        if self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#half == Half::Top { return 15300; }
        if self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#half == Half::Bottom { return 15313; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 15333; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Top && self.r#shape == Shape::OuterRight { return 15303; }
        if self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft && self.r#half == Half::Top && self.r#waterlogged == true { return 15256; }
        if self.r#facing == Facing::North && self.r#waterlogged == true && self.r#shape == Shape::OuterRight && self.r#half == Half::Top { return 15262; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft { return 15277; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 15281; }
        if self.r#facing == Facing::North && self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom && self.r#waterlogged == false { return 15267; }
        if self.r#shape == Shape::Straight && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#waterlogged == true { return 15284; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#facing == Facing::West && self.r#shape == Shape::InnerRight { return 15298; }
        if self.r#facing == Facing::South && self.r#shape == Shape::InnerRight && self.r#waterlogged == false && self.r#half == Half::Top { return 15279; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#shape == Shape::OuterRight { return 15272; }
        if self.r#waterlogged == false && self.r#shape == Shape::OuterRight && self.r#facing == Facing::South && self.r#half == Half::Bottom { return 15293; }
        if self.r#shape == Shape::InnerLeft && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::East { return 15326; }
        if self.r#waterlogged == false && self.r#facing == Facing::East && self.r#shape == Shape::Straight && self.r#half == Half::Top { return 15315; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Bottom { return 15324; }
        if self.r#half == Half::Bottom && self.r#shape == Shape::OuterRight && self.r#facing == Facing::North && self.r#waterlogged == false { return 15273; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 15301; }
        if self.r#half == Half::Top && self.r#facing == Facing::West && self.r#shape == Shape::OuterRight && self.r#waterlogged == true { return 15302; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::Straight { return 15304; }
        if self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#waterlogged == false { return 15329; }
        if self.r#facing == Facing::South && self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::OuterRight { return 15282; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom { return 15288; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#waterlogged == false && self.r#shape == Shape::InnerRight { return 15259; }
        if self.r#facing == Facing::North && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true && self.r#half == Half::Top { return 15260; }
        if self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#waterlogged == false && self.r#facing == Facing::South { return 15283; }
        if self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == false { return 15325; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 15330; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#shape == Shape::Straight { return 15265; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#shape == Shape::OuterLeft { return 15280; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::InnerRight && self.r#facing == Facing::North { return 15268; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Top && self.r#shape == Shape::Straight { return 15294; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::InnerRight && self.r#waterlogged == true { return 15308; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::OuterLeft && self.r#waterlogged == true { return 15310; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Top { return 15316; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::InnerRight { return 15289; }
        if self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#half == Half::Top { return 15255; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#shape == Shape::OuterRight && self.r#waterlogged == false { return 15263; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#shape == Shape::Straight && self.r#waterlogged == false { return 15285; }
        if self.r#facing == Facing::East && self.r#waterlogged == false && self.r#half == Half::Top && self.r#shape == Shape::OuterRight { return 15323; }
        if self.r#half == Half::Top && self.r#shape == Shape::Straight && self.r#waterlogged == false && self.r#facing == Facing::South { return 15275; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::South && self.r#shape == Shape::OuterRight { return 15292; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#shape == Shape::OuterLeft && self.r#waterlogged == false { return 15261; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#shape == Shape::Straight && self.r#facing == Facing::East { return 15314; }
        if self.r#shape == Shape::OuterLeft && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#half == Half::Bottom { return 15291; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#shape == Shape::OuterLeft && self.r#facing == Facing::North { return 15270; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#shape == Shape::InnerRight && self.r#half == Half::Bottom { return 15309; }
        if self.r#shape == Shape::InnerLeft && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Bottom { return 15287; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#shape == Shape::InnerLeft { return 15307; }
        if self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#shape == Shape::InnerRight { return 15328; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#shape == Shape::InnerLeft && self.r#facing == Facing::East { return 15327; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15276 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15290 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 15257 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15295 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15299 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 15320 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 15266 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15271 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15306 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15278 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15274 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15332 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 15317 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15319 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15258 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 15264 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 15305 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15312 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 15296 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 15322 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 15318 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 15254 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 15269 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15311 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15286 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 15297 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 15321 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15331 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 15300 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 15313 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15333 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 15303 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15256 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 15262 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 15277 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15281 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 15267 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 15284 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 15298 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
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
        if state_id == 15272 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15293 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15326 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 15315 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 15324 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15273 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 15301 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 15302 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15304 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15329 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 15282 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15288 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15259 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15260 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 15283 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 15325 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 15330 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 15265 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15280 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 15268 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
            });
        }
        if state_id == 15294 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 15308 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 15310 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 15316 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 15289 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15255 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 15263 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 15285 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 15323 {
            return Some(MossyStoneBrickStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15275 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 15292 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 15261 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 15314 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
            });
        }
        if state_id == 15291 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15270 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 15309 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15287 {
            return Some(MossyStoneBrickStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 15307 {
            return Some(MossyStoneBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 15328 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 15327 {
            return Some(MossyStoneBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
            });
        }
        return None;
    }
}

