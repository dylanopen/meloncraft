use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherryTrapdoor {
    pub powered: bool,
    pub r#half: Half,
    pub waterlogged: bool,
    pub open: bool,
    pub r#facing: Facing,
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

impl BlockState for CherryTrapdoor {
    fn to_id(self) -> i32 {
        if block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 7238; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::West { return 7274; }
        if block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#powered == false { return 7280; }
        if block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#powered == false { return 7248; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 7276; }
        if block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#powered == false { return 7263; }
        if block_state.r#open == false && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 7239; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South { return 7259; }
        if block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#facing == Facing::West { return 7275; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 7283; }
        if block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#half == Half::Top { return 7236; }
        if block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 7284; }
        if block_state.r#powered == true && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 7265; }
        if block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#half == Half::Bottom { return 7295; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#open == true { return 7242; }
        if block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 7249; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West { return 7273; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == false { return 7243; }
        if block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#waterlogged == true { return 7237; }
        if block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 7245; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 7296; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#open == false { return 7256; }
        if block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#powered == true { return 7266; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#open == true { return 7281; }
        if block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#powered == false { return 7271; }
        if block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 7233; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#facing == Facing::South { return 7261; }
        if block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#waterlogged == false { return 7270; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#open == false { return 7294; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#open == false && block_state.r#waterlogged == false { return 7254; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == true && block_state.r#waterlogged == true { return 7257; }
        if block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#facing == Facing::South { return 7250; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#powered == false { return 7247; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == true { return 7251; }
        if block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#powered == false { return 7235; }
        if block_state.r#powered == true && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 7246; }
        if block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#facing == Facing::East { return 7286; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::West { return 7268; }
        if block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == true { return 7269; }
        if block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#waterlogged == false { return 7282; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#powered == true && block_state.r#waterlogged == true { return 7241; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#waterlogged == false { return 7290; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#open == false { return 7279; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == true { return 7277; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#half == Half::Bottom { return 7292; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == true { return 7255; }
        if block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North { return 7244; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#open == false { return 7264; }
        if block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#open == false { return 7285; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#open == false { return 7253; }
        if block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#waterlogged == false { return 7252; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#open == true { return 7260; }
        if block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 7287; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 7258; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 7293; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == false { return 7272; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == true { return 7267; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#open == false { return 7262; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#open == true { return 7234; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::North { return 7240; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#open == true { return 7289; }
        if block_state.r#powered == true && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 7278; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == false { return 7288; }
        if block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == true { return 7291; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7238 {
            return Some(CherryTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#open: false,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 7274 {
            return Some(CherryTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 7280 {
            return Some(CherryTrapdoor {
                r#open: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 7248 {
            return Some(CherryTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 7276 {
            return Some(CherryTrapdoor {
                r#powered: false,
                r#facing: Facing::West,
                r#open: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7263 {
            return Some(CherryTrapdoor {
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 7239 {
            return Some(CherryTrapdoor {
                r#open: false,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 7259 {
            return Some(CherryTrapdoor {
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 7275 {
            return Some(CherryTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 7283 {
            return Some(CherryTrapdoor {
                r#facing: Facing::East,
                r#open: true,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 7236 {
            return Some(CherryTrapdoor {
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 7284 {
            return Some(CherryTrapdoor {
                r#open: true,
                r#half: Half::Top,
                r#powered: false,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 7265 {
            return Some(CherryTrapdoor {
                r#powered: true,
                r#open: true,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 7295 {
            return Some(CherryTrapdoor {
                r#open: false,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#powered: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7242 {
            return Some(CherryTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 7249 {
            return Some(CherryTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 7273 {
            return Some(CherryTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#open: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 7243 {
            return Some(CherryTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 7237 {
            return Some(CherryTrapdoor {
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7245 {
            return Some(CherryTrapdoor {
                r#powered: true,
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 7296 {
            return Some(CherryTrapdoor {
                r#facing: Facing::East,
                r#powered: false,
                r#open: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 7256 {
            return Some(CherryTrapdoor {
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 7266 {
            return Some(CherryTrapdoor {
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 7281 {
            return Some(CherryTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::East,
                r#open: true,
            });
        }
        if state_id == 7271 {
            return Some(CherryTrapdoor {
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 7233 {
            return Some(CherryTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#powered: true,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 7261 {
            return Some(CherryTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 7270 {
            return Some(CherryTrapdoor {
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7294 {
            return Some(CherryTrapdoor {
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 7254 {
            return Some(CherryTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: true,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7257 {
            return Some(CherryTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7250 {
            return Some(CherryTrapdoor {
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 7247 {
            return Some(CherryTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 7251 {
            return Some(CherryTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7235 {
            return Some(CherryTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 7246 {
            return Some(CherryTrapdoor {
                r#powered: true,
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 7286 {
            return Some(CherryTrapdoor {
                r#waterlogged: false,
                r#powered: true,
                r#half: Half::Top,
                r#open: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 7268 {
            return Some(CherryTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 7269 {
            return Some(CherryTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7282 {
            return Some(CherryTrapdoor {
                r#open: true,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7241 {
            return Some(CherryTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7290 {
            return Some(CherryTrapdoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 7279 {
            return Some(CherryTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 7277 {
            return Some(CherryTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: true,
            });
        }
        if state_id == 7292 {
            return Some(CherryTrapdoor {
                r#powered: false,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7255 {
            return Some(CherryTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#open: false,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7244 {
            return Some(CherryTrapdoor {
                r#powered: false,
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 7264 {
            return Some(CherryTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 7285 {
            return Some(CherryTrapdoor {
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 7253 {
            return Some(CherryTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 7252 {
            return Some(CherryTrapdoor {
                r#open: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7260 {
            return Some(CherryTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 7287 {
            return Some(CherryTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 7258 {
            return Some(CherryTrapdoor {
                r#open: true,
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7293 {
            return Some(CherryTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 7272 {
            return Some(CherryTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 7267 {
            return Some(CherryTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7262 {
            return Some(CherryTrapdoor {
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 7234 {
            return Some(CherryTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 7240 {
            return Some(CherryTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#powered: false,
                r#open: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 7289 {
            return Some(CherryTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 7278 {
            return Some(CherryTrapdoor {
                r#powered: true,
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 7288 {
            return Some(CherryTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 7291 {
            return Some(CherryTrapdoor {
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        return None;
    }
}

