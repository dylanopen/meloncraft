use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaTrapdoor {
    pub r#half: Half,
    pub powered: bool,
    pub waterlogged: bool,
    pub r#facing: Facing,
    pub open: bool,
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

impl BlockState for AcaciaTrapdoor {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == false { return 7183; }
        if block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#waterlogged == true { return 7185; }
        if block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::North { return 7170; }
        if block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == false { return 7184; }
        if block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 7221; }
        if block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#powered == false { return 7187; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#half == Half::Top { return 7188; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == false { return 7196; }
        if block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#waterlogged == false { return 7174; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#open == true && block_state.r#half == Half::Bottom { return 7212; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::West { return 7207; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom { return 7195; }
        if block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#waterlogged == true { return 7213; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#powered == true { return 7218; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 7224; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#open == false { return 7189; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#powered == false { return 7172; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#powered == false { return 7227; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#facing == Facing::East && block_state.r#powered == false { return 7232; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#open == true && block_state.r#waterlogged == true { return 7179; }
        if block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#powered == true { return 7182; }
        if block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#facing == Facing::South { return 7191; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#waterlogged == false { return 7214; }
        if block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#facing == Facing::South { return 7190; }
        if block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#open == false { return 7181; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == false { return 7208; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 7200; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#powered == false { return 7192; }
        if block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#waterlogged == true { return 7197; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#open == true { return 7226; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#powered == false { return 7171; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#open == false { return 7222; }
        if block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 7229; }
        if block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#waterlogged == true { return 7217; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#waterlogged == false { return 7178; }
        if block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top { return 7202; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#powered == false { return 7220; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#facing == Facing::South { return 7194; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#half == Half::Bottom { return 7209; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == true { return 7177; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#open == false && block_state.r#waterlogged == true { return 7231; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#powered == true && block_state.r#waterlogged == true { return 7201; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#open == false { return 7216; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#open == false { return 7215; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#facing == Facing::South { return 7193; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == true && block_state.r#waterlogged == false { return 7210; }
        if block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Top { return 7175; }
        if block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#open == true { return 7203; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#open == true { return 7219; }
        if block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#powered == true { return 7205; }
        if block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#open == true { return 7180; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#open == true && block_state.r#waterlogged == true { return 7169; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#open == false { return 7198; }
        if block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#waterlogged == true { return 7199; }
        if block_state.r#powered == true && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 7186; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#open == false { return 7173; }
        if block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#waterlogged == false { return 7204; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == true && block_state.r#waterlogged == true { return 7225; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == false { return 7230; }
        if block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#powered == false { return 7176; }
        if block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#facing == Facing::West { return 7206; }
        if block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 7223; }
        if block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#powered == false { return 7211; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == false { return 7228; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7183 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 7185 {
            return Some(AcaciaTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::South,
                r#waterlogged: true,
            });
        }
        if state_id == 7170 {
            return Some(AcaciaTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 7184 {
            return Some(AcaciaTrapdoor {
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7221 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 7187 {
            return Some(AcaciaTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 7188 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 7196 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 7174 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::North,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7212 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: false,
                r#open: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7207 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: false,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 7195 {
            return Some(AcaciaTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7213 {
            return Some(AcaciaTrapdoor {
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7218 {
            return Some(AcaciaTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 7224 {
            return Some(AcaciaTrapdoor {
                r#powered: false,
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 7189 {
            return Some(AcaciaTrapdoor {
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 7172 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 7227 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 7232 {
            return Some(AcaciaTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 7179 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: false,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7182 {
            return Some(AcaciaTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#powered: true,
            });
        }
        if state_id == 7191 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 7214 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#facing: Facing::West,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7190 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 7181 {
            return Some(AcaciaTrapdoor {
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 7208 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 7200 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::South,
                r#powered: false,
                r#open: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7192 {
            return Some(AcaciaTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 7197 {
            return Some(AcaciaTrapdoor {
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7226 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 7171 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: true,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 7222 {
            return Some(AcaciaTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 7229 {
            return Some(AcaciaTrapdoor {
                r#open: false,
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 7217 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7178 {
            return Some(AcaciaTrapdoor {
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7202 {
            return Some(AcaciaTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 7220 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#open: true,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 7194 {
            return Some(AcaciaTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 7209 {
            return Some(AcaciaTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#open: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7177 {
            return Some(AcaciaTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 7231 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#powered: false,
                r#open: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7201 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#open: true,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7216 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 7215 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 7193 {
            return Some(AcaciaTrapdoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 7210 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7175 {
            return Some(AcaciaTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Top,
            });
        }
        if state_id == 7203 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 7219 {
            return Some(AcaciaTrapdoor {
                r#powered: false,
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 7205 {
            return Some(AcaciaTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 7180 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 7169 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#powered: true,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7198 {
            return Some(AcaciaTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 7199 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7186 {
            return Some(AcaciaTrapdoor {
                r#powered: true,
                r#open: true,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 7173 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 7204 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: false,
            });
        }
        if state_id == 7225 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7230 {
            return Some(AcaciaTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7176 {
            return Some(AcaciaTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 7206 {
            return Some(AcaciaTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 7223 {
            return Some(AcaciaTrapdoor {
                r#powered: false,
                r#facing: Facing::East,
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 7211 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 7228 {
            return Some(AcaciaTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        return None;
    }
}

