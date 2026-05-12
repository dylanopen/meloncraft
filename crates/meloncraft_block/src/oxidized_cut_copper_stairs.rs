use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OxidizedCutCopperStairs {
    pub r#facing: Facing,
    pub r#half: Half,
    pub r#shape: Shape,
    pub waterlogged: bool,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Straight,
    InnerLeft,
    InnerRight,
    OuterLeft,
    OuterRight,
}

impl BlockState for OxidizedCutCopperStairs {
    fn to_id(self) -> i32 {
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false { return 25128; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight { return 25144; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 25179; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterLeft { return 25142; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 25150; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::West { return 25173; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft { return 25181; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top { return 25188; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 25199; }
        if block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#shape == Shape::OuterLeft { return 25131; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom { return 25156; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true { return 25127; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft { return 25178; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 25193; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true { return 25167; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#facing == Facing::West { return 25169; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::East { return 25204; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight { return 25154; }
        if block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 25145; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 25201; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight { return 25166; }
        if block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 25170; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East { return 25202; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::North { return 25126; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#waterlogged == false { return 25136; }
        if block_state.r#facing == Facing::North && block_state.r#shape == Shape::Straight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 25135; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == false { return 25160; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::West { return 25180; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true { return 25159; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::East { return 25185; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterRight { return 25184; }
        if block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::South { return 25147; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 25141; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 25132; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Top { return 25172; }
        if block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 25186; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 25196; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#shape == Shape::InnerLeft { return 25177; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true { return 25163; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::OuterLeft { return 25162; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 25191; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerRight { return 25140; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 25189; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#shape == Shape::Straight { return 25125; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#shape == Shape::InnerRight && block_state.r#waterlogged == true { return 25129; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 25192; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::InnerLeft && block_state.r#waterlogged == false { return 25198; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Bottom { return 25200; }
        if block_state.r#shape == Shape::OuterLeft && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 25161; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 25171; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East { return 25203; }
        if block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 25134; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::East { return 25190; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 25133; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::North { return 25139; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 25158; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Top { return 25168; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight { return 25175; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::South { return 25146; }
        if block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerRight && block_state.r#facing == Facing::South && block_state.r#half == Half::Top { return 25149; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West { return 25183; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::East { return 25194; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight { return 25153; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#shape == Shape::InnerLeft { return 25187; }
        if block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterRight && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 25164; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::East && block_state.r#waterlogged == true { return 25195; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterLeft { return 25182; }
        if block_state.r#shape == Shape::InnerRight && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 25130; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerLeft { return 25157; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#shape == Shape::InnerLeft { return 25148; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == false { return 25174; }
        if block_state.r#waterlogged == false && block_state.r#shape == Shape::OuterLeft && block_state.r#facing == Facing::South && block_state.r#half == Half::Top { return 25152; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#shape == Shape::OuterLeft { return 25151; }
        if block_state.r#shape == Shape::OuterRight && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North { return 25143; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#shape == Shape::InnerLeft { return 25138; }
        if block_state.r#half == Half::Bottom && block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 25176; }
        if block_state.r#shape == Shape::Straight && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 25165; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#shape == Shape::InnerLeft { return 25137; }
        if block_state.r#shape == Shape::Straight && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 25155; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#shape == Shape::InnerLeft && block_state.r#half == Half::Bottom { return 25197; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25128 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 25144 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25179 {
            return Some(OxidizedCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 25142 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25150 {
            return Some(OxidizedCutCopperStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 25173 {
            return Some(OxidizedCutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 25181 {
            return Some(OxidizedCutCopperStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25188 {
            return Some(OxidizedCutCopperStairs {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 25199 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 25131 {
            return Some(OxidizedCutCopperStairs {
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25156 {
            return Some(OxidizedCutCopperStairs {
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25127 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 25178 {
            return Some(OxidizedCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25193 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 25167 {
            return Some(OxidizedCutCopperStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
            });
        }
        if state_id == 25169 {
            return Some(OxidizedCutCopperStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 25204 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 25154 {
            return Some(OxidizedCutCopperStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25145 {
            return Some(OxidizedCutCopperStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 25201 {
            return Some(OxidizedCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25166 {
            return Some(OxidizedCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25170 {
            return Some(OxidizedCutCopperStairs {
                r#facing: Facing::West,
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25202 {
            return Some(OxidizedCutCopperStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 25126 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::Straight,
                r#facing: Facing::North,
            });
        }
        if state_id == 25136 {
            return Some(OxidizedCutCopperStairs {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#waterlogged: false,
            });
        }
        if state_id == 25135 {
            return Some(OxidizedCutCopperStairs {
                r#facing: Facing::North,
                r#shape: Shape::Straight,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25160 {
            return Some(OxidizedCutCopperStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: false,
            });
        }
        if state_id == 25180 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::West,
            });
        }
        if state_id == 25159 {
            return Some(OxidizedCutCopperStairs {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 25185 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
            });
        }
        if state_id == 25184 {
            return Some(OxidizedCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25147 {
            return Some(OxidizedCutCopperStairs {
                r#shape: Shape::InnerLeft,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
            });
        }
        if state_id == 25141 {
            return Some(OxidizedCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25132 {
            return Some(OxidizedCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 25172 {
            return Some(OxidizedCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 25186 {
            return Some(OxidizedCutCopperStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 25196 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 25177 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25163 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
            });
        }
        if state_id == 25162 {
            return Some(OxidizedCutCopperStairs {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25191 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 25140 {
            return Some(OxidizedCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#shape: Shape::InnerRight,
            });
        }
        if state_id == 25189 {
            return Some(OxidizedCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 25125 {
            return Some(OxidizedCutCopperStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: true,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25129 {
            return Some(OxidizedCutCopperStairs {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#shape: Shape::InnerRight,
                r#waterlogged: true,
            });
        }
        if state_id == 25192 {
            return Some(OxidizedCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 25198 {
            return Some(OxidizedCutCopperStairs {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#shape: Shape::InnerLeft,
                r#waterlogged: false,
            });
        }
        if state_id == 25200 {
            return Some(OxidizedCutCopperStairs {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#shape: Shape::InnerRight,
                r#half: Half::Bottom,
            });
        }
        if state_id == 25161 {
            return Some(OxidizedCutCopperStairs {
                r#shape: Shape::OuterLeft,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 25171 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 25203 {
            return Some(OxidizedCutCopperStairs {
                r#waterlogged: true,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#facing: Facing::East,
            });
        }
        if state_id == 25134 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 25190 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#shape: Shape::InnerRight,
                r#facing: Facing::East,
            });
        }
        if state_id == 25133 {
            return Some(OxidizedCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 25139 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::North,
            });
        }
        if state_id == 25158 {
            return Some(OxidizedCutCopperStairs {
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 25168 {
            return Some(OxidizedCutCopperStairs {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
                r#half: Half::Top,
            });
        }
        if state_id == 25175 {
            return Some(OxidizedCutCopperStairs {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
            });
        }
        if state_id == 25146 {
            return Some(OxidizedCutCopperStairs {
                r#waterlogged: false,
                r#half: Half::Top,
                r#shape: Shape::Straight,
                r#facing: Facing::South,
            });
        }
        if state_id == 25149 {
            return Some(OxidizedCutCopperStairs {
                r#waterlogged: true,
                r#shape: Shape::InnerRight,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 25183 {
            return Some(OxidizedCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 25194 {
            return Some(OxidizedCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::East,
            });
        }
        if state_id == 25153 {
            return Some(OxidizedCutCopperStairs {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
            });
        }
        if state_id == 25187 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25164 {
            return Some(OxidizedCutCopperStairs {
                r#facing: Facing::South,
                r#shape: Shape::OuterRight,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 25195 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 25182 {
            return Some(OxidizedCutCopperStairs {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25130 {
            return Some(OxidizedCutCopperStairs {
                r#shape: Shape::InnerRight,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 25157 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25148 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25174 {
            return Some(OxidizedCutCopperStairs {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#shape: Shape::OuterRight,
                r#waterlogged: false,
            });
        }
        if state_id == 25152 {
            return Some(OxidizedCutCopperStairs {
                r#waterlogged: false,
                r#shape: Shape::OuterLeft,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 25151 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#shape: Shape::OuterLeft,
            });
        }
        if state_id == 25143 {
            return Some(OxidizedCutCopperStairs {
                r#shape: Shape::OuterRight,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 25138 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25176 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Bottom,
                r#shape: Shape::Straight,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 25165 {
            return Some(OxidizedCutCopperStairs {
                r#shape: Shape::Straight,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 25137 {
            return Some(OxidizedCutCopperStairs {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#shape: Shape::InnerLeft,
            });
        }
        if state_id == 25155 {
            return Some(OxidizedCutCopperStairs {
                r#shape: Shape::Straight,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 25197 {
            return Some(OxidizedCutCopperStairs {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#shape: Shape::InnerLeft,
                r#half: Half::Bottom,
            });
        }
        return None;
    }
}

