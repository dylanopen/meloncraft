use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuffBrickStairs {
    pub waterlogged: bool,
    pub r#half: Half,
    pub r#facing: Facing,
    pub r#shape: Shape,
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

impl BlockState for TuffBrickStairs {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 24118; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#shape == Shape::Straight { return 24140; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false { return 24143; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true { return 24152; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 24122; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight { return 24081; }
        if block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 24090; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 24082; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 24116; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom { return 24138; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 24092; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight { return 24085; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 24091; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 24108; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterLeft { return 24117; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::West { return 24128; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 24151; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterRight { return 24159; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false { return 24129; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight { return 24098; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight { return 24104; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 24095; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 24109; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::South { return 24110; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::West { return 24133; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 24150; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight { return 24130; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 24142; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false { return 24155; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false { return 24103; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::West { return 24123; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerRight { return 24125; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top { return 24124; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top { return 24141; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top { return 24149; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight { return 24148; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom { return 24137; }
        if block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 24144; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::West { return 24132; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true { return 24100; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 24119; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom { return 24113; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 24088; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true { return 24158; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::South { return 24112; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight { return 24114; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom { return 24134; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false { return 24131; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerRight { return 24145; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom { return 24154; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom { return 24093; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 24115; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterLeft { return 24107; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 24083; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 24087; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Top { return 24089; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 24146; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom { return 24111; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false { return 24121; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom { return 24094; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::South { return 24102; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom { return 24156; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false { return 24147; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 24086; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom { return 24139; }
        if block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 24101; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft { return 24127; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 24135; }
        if block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 24084; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterLeft { return 24126; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft { return 24153; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top { return 24120; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft { return 24157; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterLeft { return 24096; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterLeft { return 24136; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterRight { return 24099; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true { return 24080; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 24105; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterLeft { return 24106; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::North { return 24097; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 24118 {
            return Some(TuffBrickStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 24140 {
            return Some(TuffBrickStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 24143 {
            return Some(TuffBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 24152 {
            return Some(TuffBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 24122 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 24081 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 24090 {
            return Some(TuffBrickStairs {
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 24082 {
            return Some(TuffBrickStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 24116 {
            return Some(TuffBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 24138 {
            return Some(TuffBrickStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 24092 {
            return Some(TuffBrickStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 24085 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 24091 {
            return Some(TuffBrickStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 24108 {
            return Some(TuffBrickStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 24117 {
            return Some(TuffBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 24128 {
            return Some(TuffBrickStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 24151 {
            return Some(TuffBrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 24159 {
            return Some(TuffBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 24129 {
            return Some(TuffBrickStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 24098 {
            return Some(TuffBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 24104 {
            return Some(TuffBrickStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 24095 {
            return Some(TuffBrickStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 24109 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 24110 {
            return Some(TuffBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
            });
        }
        if state_id == 24133 {
            return Some(TuffBrickStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 24150 {
            return Some(TuffBrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 24130 {
            return Some(TuffBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 24142 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 24155 {
            return Some(TuffBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 24103 {
            return Some(TuffBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 24123 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 24125 {
            return Some(TuffBrickStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 24124 {
            return Some(TuffBrickStairs {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 24141 {
            return Some(TuffBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 24149 {
            return Some(TuffBrickStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
            });
        }
        if state_id == 24148 {
            return Some(TuffBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 24137 {
            return Some(TuffBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 24144 {
            return Some(TuffBrickStairs {
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 24132 {
            return Some(TuffBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
            });
        }
        if state_id == 24100 {
            return Some(TuffBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 24119 {
            return Some(TuffBrickStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 24113 {
            return Some(TuffBrickStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 24088 {
            return Some(TuffBrickStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 24158 {
            return Some(TuffBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 24112 {
            return Some(TuffBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 24114 {
            return Some(TuffBrickStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 24134 {
            return Some(TuffBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 24131 {
            return Some(TuffBrickStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 24145 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 24154 {
            return Some(TuffBrickStairs {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 24093 {
            return Some(TuffBrickStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 24115 {
            return Some(TuffBrickStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 24107 {
            return Some(TuffBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 24083 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 24087 {
            return Some(TuffBrickStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 24089 {
            return Some(TuffBrickStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 24146 {
            return Some(TuffBrickStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 24111 {
            return Some(TuffBrickStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 24121 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 24094 {
            return Some(TuffBrickStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 24102 {
            return Some(TuffBrickStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::South,
            });
        }
        if state_id == 24156 {
            return Some(TuffBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 24147 {
            return Some(TuffBrickStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 24086 {
            return Some(TuffBrickStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 24139 {
            return Some(TuffBrickStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 24101 {
            return Some(TuffBrickStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 24127 {
            return Some(TuffBrickStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 24135 {
            return Some(TuffBrickStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 24084 {
            return Some(TuffBrickStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 24126 {
            return Some(TuffBrickStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 24153 {
            return Some(TuffBrickStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 24120 {
            return Some(TuffBrickStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#half: Half::Top,
            });
        }
        if state_id == 24157 {
            return Some(TuffBrickStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 24096 {
            return Some(TuffBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 24136 {
            return Some(TuffBrickStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 24099 {
            return Some(TuffBrickStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 24080 {
            return Some(TuffBrickStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
            });
        }
        if state_id == 24105 {
            return Some(TuffBrickStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 24106 {
            return Some(TuffBrickStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 24097 {
            return Some(TuffBrickStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
            });
        }
        return None;
    }
}

