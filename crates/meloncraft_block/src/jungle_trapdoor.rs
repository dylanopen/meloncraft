use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleTrapdoor {
    pub open: bool,
    pub waterlogged: bool,
    pub r#facing: Facing,
    pub r#half: Half,
    pub powered: bool,
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

impl BlockState for JungleTrapdoor {
    fn to_id(self) -> i32 {
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#open == true { return 7124; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#open == true { return 7129; }
        if block_state.r#facing == Facing::North && block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Bottom { return 7113; }
        if block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#facing == Facing::North { return 7116; }
        if block_state.r#powered == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#waterlogged == false { return 7126; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#powered == true { return 7165; }
        if block_state.r#powered == true && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#facing == Facing::East { return 7154; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 7156; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == true { return 7147; }
        if block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::North { return 7115; }
        if block_state.r#open == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#waterlogged == false { return 7122; }
        if block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#powered == false { return 7135; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#powered == true { return 7141; }
        if block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 7150; }
        if block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#waterlogged == true { return 7159; }
        if block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#waterlogged == true { return 7139; }
        if block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#waterlogged == true { return 7123; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == true { return 7107; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#open == false { return 7144; }
        if block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 7149; }
        if block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 7117; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#waterlogged == false && block_state.r#open == false { return 7136; }
        if block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false { return 7148; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::West { return 7152; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::West { return 7137; }
        if block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom { return 7133; }
        if block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom { return 7118; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == false { return 7168; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == false { return 7164; }
        if block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 7111; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#open == false && block_state.r#half == Half::Top { return 7125; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom { return 7119; }
        if block_state.r#facing == Facing::West && block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#powered == true { return 7138; }
        if block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 7114; }
        if block_state.r#powered == false && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#half == Half::Top { return 7140; }
        if block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#powered == true { return 7146; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#facing == Facing::West && block_state.r#waterlogged == true && block_state.r#open == true { return 7145; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#open == false { return 7151; }
        if block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#powered == false { return 7120; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#open == true && block_state.r#powered == true && block_state.r#half == Half::Bottom { return 7162; }
        if block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 7160; }
        if block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == true { return 7163; }
        if block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#powered == false { return 7128; }
        if block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 7157; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#open == true { return 7105; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#open == true { return 7161; }
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::South { return 7131; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#open == true { return 7106; }
        if block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 7112; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#open == false { return 7166; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#waterlogged == true { return 7143; }
        if block_state.r#powered == true && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 7110; }
        if block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#powered == false && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom { return 7167; }
        if block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#powered == false { return 7127; }
        if block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#waterlogged == false { return 7158; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#open == true && block_state.r#waterlogged == true { return 7121; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 7132; }
        if block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#waterlogged == false { return 7108; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#powered == true && block_state.r#facing == Facing::West { return 7142; }
        if block_state.r#open == true && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#facing == Facing::East { return 7153; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 7155; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#half == Half::Top { return 7109; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#open == false { return 7134; }
        if block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 7130; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7124 {
            return Some(JungleTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 7129 {
            return Some(JungleTrapdoor {
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 7113 {
            return Some(JungleTrapdoor {
                r#facing: Facing::North,
                r#waterlogged: true,
                r#open: true,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7116 {
            return Some(JungleTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 7126 {
            return Some(JungleTrapdoor {
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7165 {
            return Some(JungleTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 7154 {
            return Some(JungleTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 7156 {
            return Some(JungleTrapdoor {
                r#powered: false,
                r#open: true,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7147 {
            return Some(JungleTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7115 {
            return Some(JungleTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 7122 {
            return Some(JungleTrapdoor {
                r#open: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7135 {
            return Some(JungleTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 7141 {
            return Some(JungleTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#open: false,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 7150 {
            return Some(JungleTrapdoor {
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7159 {
            return Some(JungleTrapdoor {
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7139 {
            return Some(JungleTrapdoor {
                r#facing: Facing::West,
                r#open: true,
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7123 {
            return Some(JungleTrapdoor {
                r#open: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7107 {
            return Some(JungleTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7144 {
            return Some(JungleTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 7149 {
            return Some(JungleTrapdoor {
                r#facing: Facing::West,
                r#open: false,
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 7117 {
            return Some(JungleTrapdoor {
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 7136 {
            return Some(JungleTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 7148 {
            return Some(JungleTrapdoor {
                r#facing: Facing::West,
                r#open: true,
                r#powered: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
            });
        }
        if state_id == 7152 {
            return Some(JungleTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 7137 {
            return Some(JungleTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#open: true,
                r#half: Half::Top,
                r#facing: Facing::West,
            });
        }
        if state_id == 7133 {
            return Some(JungleTrapdoor {
                r#open: false,
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7118 {
            return Some(JungleTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7168 {
            return Some(JungleTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 7164 {
            return Some(JungleTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7111 {
            return Some(JungleTrapdoor {
                r#open: false,
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 7125 {
            return Some(JungleTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7119 {
            return Some(JungleTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7138 {
            return Some(JungleTrapdoor {
                r#facing: Facing::West,
                r#open: true,
                r#waterlogged: false,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 7114 {
            return Some(JungleTrapdoor {
                r#open: true,
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 7140 {
            return Some(JungleTrapdoor {
                r#powered: false,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Top,
            });
        }
        if state_id == 7146 {
            return Some(JungleTrapdoor {
                r#facing: Facing::West,
                r#waterlogged: false,
                r#open: true,
                r#half: Half::Bottom,
                r#powered: true,
            });
        }
        if state_id == 7145 {
            return Some(JungleTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 7151 {
            return Some(JungleTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: true,
                r#open: false,
            });
        }
        if state_id == 7120 {
            return Some(JungleTrapdoor {
                r#open: false,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 7162 {
            return Some(JungleTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: false,
                r#open: true,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7160 {
            return Some(JungleTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7163 {
            return Some(JungleTrapdoor {
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7128 {
            return Some(JungleTrapdoor {
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 7157 {
            return Some(JungleTrapdoor {
                r#open: false,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 7105 {
            return Some(JungleTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 7161 {
            return Some(JungleTrapdoor {
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 7131 {
            return Some(JungleTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#powered: false,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 7106 {
            return Some(JungleTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 7112 {
            return Some(JungleTrapdoor {
                r#facing: Facing::North,
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7166 {
            return Some(JungleTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#waterlogged: false,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 7143 {
            return Some(JungleTrapdoor {
                r#powered: false,
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 7110 {
            return Some(JungleTrapdoor {
                r#powered: true,
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 7167 {
            return Some(JungleTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#powered: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7127 {
            return Some(JungleTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 7158 {
            return Some(JungleTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7121 {
            return Some(JungleTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: true,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7132 {
            return Some(JungleTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 7108 {
            return Some(JungleTrapdoor {
                r#open: true,
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 7142 {
            return Some(JungleTrapdoor {
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: false,
                r#powered: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 7153 {
            return Some(JungleTrapdoor {
                r#open: true,
                r#waterlogged: true,
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 7155 {
            return Some(JungleTrapdoor {
                r#powered: false,
                r#open: true,
                r#half: Half::Top,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 7109 {
            return Some(JungleTrapdoor {
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7134 {
            return Some(JungleTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#open: false,
            });
        }
        if state_id == 7130 {
            return Some(JungleTrapdoor {
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        return None;
    }
}

