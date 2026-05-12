use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonStairs {
    pub r#half: Half,
    pub r#facing: Facing,
    pub waterlogged: bool,
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

impl BlockState for CrimsonStairs {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false { return 21127; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false { return 21165; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterLeft { return 21161; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight { return 21109; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom { return 21139; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::North { return 21108; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top { return 21148; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top { return 21129; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true { return 21130; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 21124; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false { return 21149; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::East { return 21172; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterLeft { return 21120; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top && block_state.r#facing == Facing::North { return 21105; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom { return 21159; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South { return 21143; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 21175; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterRight { return 21132; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false { return 21107; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::South { return 21125; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 21140; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 21133; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerLeft { return 21136; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false { return 21141; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 21163; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 21169; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom { return 21119; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::East { return 21170; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterLeft { return 21180; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top { return 21111; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false && block_state.r#facing == Facing::West { return 21153; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::South && block_state.r#half == Half::Top { return 21128; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom { return 21142; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true { return 21162; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom { return 21117; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::North { return 21121; }
        if block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 21144; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom { return 21181; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight { return 21158; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 21168; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight { return 21183; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top { return 21166; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top && block_state.r#facing == Facing::North { return 21113; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom { return 21134; }
        if block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 21106; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top && block_state.r#facing == Facing::North { return 21104; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom { return 21114; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true { return 21178; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South { return 21137; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#shape == Shape::Straight { return 21155; }
        if block_state.r#shape == Shape::Straight && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::East { return 21164; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom { return 21174; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 21122; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight { return 21182; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false { return 21179; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::East { return 21173; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft { return 21126; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top && block_state.r#facing == Facing::East { return 21167; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 21147; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft { return 21176; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false && block_state.r#facing == Facing::East { return 21171; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 21116; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft { return 21177; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterLeft { return 21110; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 21157; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 21151; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom { return 21118; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterLeft { return 21131; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 21152; }
        if block_state.r#facing == Facing::North && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 21115; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight { return 21123; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::West { return 21150; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 21160; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 21135; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true { return 21138; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 21156; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterRight { return 21112; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false { return 21145; }
        if block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 21154; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerLeft { return 21146; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21127 {
            return Some(CrimsonStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 21165 {
            return Some(CrimsonStairs {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 21161 {
            return Some(CrimsonStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 21109 {
            return Some(CrimsonStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 21139 {
            return Some(CrimsonStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21108 {
            return Some(CrimsonStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
            });
        }
        if state_id == 21148 {
            return Some(CrimsonStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 21129 {
            return Some(CrimsonStairs {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
            });
        }
        if state_id == 21130 {
            return Some(CrimsonStairs {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 21124 {
            return Some(CrimsonStairs {
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 21149 {
            return Some(CrimsonStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 21172 {
            return Some(CrimsonStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 21120 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 21105 {
            return Some(CrimsonStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 21159 {
            return Some(CrimsonStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21143 {
            return Some(CrimsonStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 21175 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 21132 {
            return Some(CrimsonStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 21107 {
            return Some(CrimsonStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 21125 {
            return Some(CrimsonStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
            });
        }
        if state_id == 21140 {
            return Some(CrimsonStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 21133 {
            return Some(CrimsonStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 21136 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 21141 {
            return Some(CrimsonStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 21163 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 21169 {
            return Some(CrimsonStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 21119 {
            return Some(CrimsonStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21170 {
            return Some(CrimsonStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
            });
        }
        if state_id == 21180 {
            return Some(CrimsonStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 21111 {
            return Some(CrimsonStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 21153 {
            return Some(CrimsonStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 21128 {
            return Some(CrimsonStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 21142 {
            return Some(CrimsonStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21162 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 21117 {
            return Some(CrimsonStairs {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21121 {
            return Some(CrimsonStairs {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
            });
        }
        if state_id == 21144 {
            return Some(CrimsonStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 21181 {
            return Some(CrimsonStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21158 {
            return Some(CrimsonStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 21168 {
            return Some(CrimsonStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 21183 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 21166 {
            return Some(CrimsonStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 21113 {
            return Some(CrimsonStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 21134 {
            return Some(CrimsonStairs {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21106 {
            return Some(CrimsonStairs {
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 21104 {
            return Some(CrimsonStairs {
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 21114 {
            return Some(CrimsonStairs {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21178 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 21137 {
            return Some(CrimsonStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 21155 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 21164 {
            return Some(CrimsonStairs {
                r#shape: Shape::Straight,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 21174 {
            return Some(CrimsonStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21122 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 21182 {
            return Some(CrimsonStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 21179 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 21173 {
            return Some(CrimsonStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 21126 {
            return Some(CrimsonStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 21167 {
            return Some(CrimsonStairs {
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 21147 {
            return Some(CrimsonStairs {
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 21176 {
            return Some(CrimsonStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 21171 {
            return Some(CrimsonStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 21116 {
            return Some(CrimsonStairs {
                r#shape: Shape::InnerLeft,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21177 {
            return Some(CrimsonStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 21110 {
            return Some(CrimsonStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 21157 {
            return Some(CrimsonStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 21151 {
            return Some(CrimsonStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 21118 {
            return Some(CrimsonStairs {
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21131 {
            return Some(CrimsonStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 21152 {
            return Some(CrimsonStairs {
                r#shape: Shape::OuterRight,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 21115 {
            return Some(CrimsonStairs {
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 21123 {
            return Some(CrimsonStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 21150 {
            return Some(CrimsonStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 21160 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 21135 {
            return Some(CrimsonStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 21138 {
            return Some(CrimsonStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 21156 {
            return Some(CrimsonStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 21112 {
            return Some(CrimsonStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 21145 {
            return Some(CrimsonStairs {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 21154 {
            return Some(CrimsonStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 21146 {
            return Some(CrimsonStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
            });
        }
        return None;
    }
}

