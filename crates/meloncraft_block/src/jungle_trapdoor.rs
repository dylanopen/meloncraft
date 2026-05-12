use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JungleTrapdoor {
    pub powered: bool,
    pub waterlogged: bool,
    pub open: bool,
    pub r#facing: Facing,
    pub r#half: Half,
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
    fn to_id(&self) -> i32 {
        if self.r#facing == Facing::West && self.r#powered == false && self.r#half == Half::Top && self.r#open == true && self.r#waterlogged == false { return 7140; }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#waterlogged == false && self.r#half == Half::Top && self.r#open == true { return 7106; }
        if self.r#facing == Facing::South && self.r#open == true && self.r#half == Half::Bottom && self.r#powered == true && self.r#waterlogged == false { return 7130; }
        if self.r#powered == false && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#open == true { return 7132; }
        if self.r#waterlogged == false && self.r#powered == false && self.r#half == Half::Top && self.r#facing == Facing::West && self.r#open == false { return 7144; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#open == true && self.r#powered == true { return 7114; }
        if self.r#powered == false && self.r#waterlogged == true && self.r#open == false && self.r#facing == Facing::South && self.r#half == Half::Top { return 7127; }
        if self.r#facing == Facing::North && self.r#open == false && self.r#powered == false && self.r#half == Half::Top && self.r#waterlogged == false { return 7112; }
        if self.r#half == Half::Bottom && self.r#open == false && self.r#facing == Facing::East && self.r#powered == false && self.r#waterlogged == true { return 7167; }
        if self.r#powered == false && self.r#open == true && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::North { return 7115; }
        if self.r#waterlogged == false && self.r#open == true && self.r#powered == false && self.r#half == Half::Bottom && self.r#facing == Facing::North { return 7116; }
        if self.r#facing == Facing::South && self.r#open == false && self.r#powered == false && self.r#waterlogged == false && self.r#half == Half::Bottom { return 7136; }
        if self.r#half == Half::Top && self.r#facing == Facing::North && self.r#open == true && self.r#powered == true && self.r#waterlogged == true { return 7105; }
        if self.r#powered == true && self.r#waterlogged == true && self.r#open == false && self.r#facing == Facing::West && self.r#half == Half::Top { return 7141; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#powered == false && self.r#open == false { return 7152; }
        if self.r#open == true && self.r#waterlogged == true && self.r#half == Half::Top && self.r#facing == Facing::South && self.r#powered == false { return 7123; }
        if self.r#powered == true && self.r#open == true && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#half == Half::Top { return 7154; }
        if self.r#waterlogged == false && self.r#open == false && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#powered == false { return 7168; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#open == false && self.r#facing == Facing::East && self.r#powered == false { return 7160; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#open == true && self.r#powered == true && self.r#waterlogged == true { return 7161; }
        if self.r#powered == true && self.r#waterlogged == false && self.r#open == true && self.r#facing == Facing::West && self.r#half == Half::Top { return 7138; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#open == false && self.r#powered == true && self.r#waterlogged == true { return 7109; }
        if self.r#open == false && self.r#powered == true && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::North { return 7117; }
        if self.r#open == true && self.r#powered == false && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == false { return 7124; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#open == true && self.r#waterlogged == true && self.r#powered == false { return 7155; }
        if self.r#half == Half::Top && self.r#powered == true && self.r#open == false && self.r#waterlogged == true && self.r#facing == Facing::East { return 7157; }
        if self.r#half == Half::Top && self.r#powered == true && self.r#waterlogged == true && self.r#open == true && self.r#facing == Facing::West { return 7137; }
        if self.r#facing == Facing::West && self.r#powered == true && self.r#waterlogged == true && self.r#open == false && self.r#half == Half::Bottom { return 7149; }
        if self.r#powered == true && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#open == true && self.r#facing == Facing::North { return 7113; }
        if self.r#half == Half::Top && self.r#powered == false && self.r#open == false && self.r#facing == Facing::West && self.r#waterlogged == true { return 7143; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#open == true && self.r#waterlogged == true && self.r#powered == true { return 7129; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::North && self.r#open == false && self.r#powered == true && self.r#waterlogged == false { return 7118; }
        if self.r#facing == Facing::West && self.r#open == true && self.r#powered == true && self.r#waterlogged == false && self.r#half == Half::Bottom { return 7146; }
        if self.r#powered == true && self.r#open == false && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#half == Half::Bottom { return 7134; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#open == false && self.r#powered == true && self.r#waterlogged == false { return 7142; }
        if self.r#half == Half::Bottom && self.r#powered == true && self.r#facing == Facing::East && self.r#open == true && self.r#waterlogged == false { return 7162; }
        if self.r#waterlogged == true && self.r#open == true && self.r#facing == Facing::East && self.r#half == Half::Bottom && self.r#powered == false { return 7163; }
        if self.r#powered == true && self.r#waterlogged == true && self.r#open == true && self.r#facing == Facing::East && self.r#half == Half::Top { return 7153; }
        if self.r#open == false && self.r#half == Half::Bottom && self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::North { return 7120; }
        if self.r#half == Half::Top && self.r#open == false && self.r#facing == Facing::East && self.r#powered == true && self.r#waterlogged == false { return 7158; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#powered == false && self.r#open == true && self.r#waterlogged == true { return 7139; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#powered == true && self.r#half == Half::Bottom && self.r#open == false { return 7133; }
        if self.r#facing == Facing::South && self.r#open == false && self.r#powered == false && self.r#waterlogged == false && self.r#half == Half::Top { return 7128; }
        if self.r#powered == false && self.r#open == true && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Bottom { return 7147; }
        if self.r#open == true && self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#powered == false { return 7108; }
        if self.r#powered == true && self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#open == false { return 7110; }
        if self.r#facing == Facing::West && self.r#open == true && self.r#powered == true && self.r#half == Half::Bottom && self.r#waterlogged == true { return 7145; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#powered == false && self.r#open == false && self.r#facing == Facing::South { return 7135; }
        if self.r#half == Half::Top && self.r#open == true && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#powered == false { return 7107; }
        if self.r#open == true && self.r#waterlogged == false && self.r#powered == false && self.r#half == Half::Bottom && self.r#facing == Facing::West { return 7148; }
        if self.r#powered == false && self.r#facing == Facing::North && self.r#open == false && self.r#half == Half::Top && self.r#waterlogged == true { return 7111; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#open == false && self.r#powered == true && self.r#waterlogged == false { return 7150; }
        if self.r#half == Half::Top && self.r#open == false && self.r#powered == false && self.r#waterlogged == true && self.r#facing == Facing::East { return 7159; }
        if self.r#powered == false && self.r#half == Half::Top && self.r#open == true && self.r#waterlogged == false && self.r#facing == Facing::East { return 7156; }
        if self.r#facing == Facing::East && self.r#open == false && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#powered == true { return 7165; }
        if self.r#waterlogged == false && self.r#half == Half::Top && self.r#open == false && self.r#facing == Facing::South && self.r#powered == true { return 7126; }
        if self.r#half == Half::Top && self.r#open == true && self.r#powered == true && self.r#facing == Facing::South && self.r#waterlogged == false { return 7122; }
        if self.r#facing == Facing::South && self.r#half == Half::Bottom && self.r#open == true && self.r#powered == false && self.r#waterlogged == true { return 7131; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#open == false && self.r#facing == Facing::North && self.r#powered == false { return 7119; }
        if self.r#facing == Facing::East && self.r#open == true && self.r#half == Half::Bottom && self.r#powered == false && self.r#waterlogged == false { return 7164; }
        if self.r#powered == false && self.r#waterlogged == true && self.r#open == false && self.r#half == Half::Bottom && self.r#facing == Facing::West { return 7151; }
        if self.r#waterlogged == true && self.r#open == false && self.r#powered == true && self.r#facing == Facing::South && self.r#half == Half::Top { return 7125; }
        if self.r#powered == true && self.r#open == true && self.r#facing == Facing::South && self.r#waterlogged == true && self.r#half == Half::Top { return 7121; }
        if self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#open == false && self.r#half == Half::Bottom { return 7166; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7140 {
            return Some(JungleTrapdoor {
                r#facing: Facing::West,
                r#powered: false,
                r#half: Half::Top,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7106 {
            return Some(JungleTrapdoor {
                r#facing: Facing::North,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 7130 {
            return Some(JungleTrapdoor {
                r#facing: Facing::South,
                r#open: true,
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7132 {
            return Some(JungleTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 7144 {
            return Some(JungleTrapdoor {
                r#waterlogged: false,
                r#powered: false,
                r#half: Half::Top,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 7114 {
            return Some(JungleTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 7127 {
            return Some(JungleTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 7112 {
            return Some(JungleTrapdoor {
                r#facing: Facing::North,
                r#open: false,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 7167 {
            return Some(JungleTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7115 {
            return Some(JungleTrapdoor {
                r#powered: false,
                r#open: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 7116 {
            return Some(JungleTrapdoor {
                r#waterlogged: false,
                r#open: true,
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::North,
            });
        }
        if state_id == 7136 {
            return Some(JungleTrapdoor {
                r#facing: Facing::South,
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7105 {
            return Some(JungleTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::North,
                r#open: true,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7141 {
            return Some(JungleTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 7152 {
            return Some(JungleTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#powered: false,
                r#open: false,
            });
        }
        if state_id == 7123 {
            return Some(JungleTrapdoor {
                r#open: true,
                r#waterlogged: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 7154 {
            return Some(JungleTrapdoor {
                r#powered: true,
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 7168 {
            return Some(JungleTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 7160 {
            return Some(JungleTrapdoor {
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 7161 {
            return Some(JungleTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#open: true,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7138 {
            return Some(JungleTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 7109 {
            return Some(JungleTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7117 {
            return Some(JungleTrapdoor {
                r#open: false,
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 7124 {
            return Some(JungleTrapdoor {
                r#open: true,
                r#powered: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
            });
        }
        if state_id == 7155 {
            return Some(JungleTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#open: true,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 7157 {
            return Some(JungleTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 7137 {
            return Some(JungleTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 7149 {
            return Some(JungleTrapdoor {
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7113 {
            return Some(JungleTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::North,
            });
        }
        if state_id == 7143 {
            return Some(JungleTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#open: false,
                r#facing: Facing::West,
                r#waterlogged: true,
            });
        }
        if state_id == 7129 {
            return Some(JungleTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: true,
                r#waterlogged: true,
                r#powered: true,
            });
        }
        if state_id == 7118 {
            return Some(JungleTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7146 {
            return Some(JungleTrapdoor {
                r#facing: Facing::West,
                r#open: true,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7134 {
            return Some(JungleTrapdoor {
                r#powered: true,
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7142 {
            return Some(JungleTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7162 {
            return Some(JungleTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#facing: Facing::East,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7163 {
            return Some(JungleTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 7153 {
            return Some(JungleTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::East,
                r#half: Half::Top,
            });
        }
        if state_id == 7120 {
            return Some(JungleTrapdoor {
                r#open: false,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 7158 {
            return Some(JungleTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7139 {
            return Some(JungleTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: false,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7133 {
            return Some(JungleTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#powered: true,
                r#half: Half::Bottom,
                r#open: false,
            });
        }
        if state_id == 7128 {
            return Some(JungleTrapdoor {
                r#facing: Facing::South,
                r#open: false,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7147 {
            return Some(JungleTrapdoor {
                r#powered: false,
                r#open: true,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7108 {
            return Some(JungleTrapdoor {
                r#open: true,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 7110 {
            return Some(JungleTrapdoor {
                r#powered: true,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        if state_id == 7145 {
            return Some(JungleTrapdoor {
                r#facing: Facing::West,
                r#open: true,
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 7135 {
            return Some(JungleTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#powered: false,
                r#open: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 7107 {
            return Some(JungleTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 7148 {
            return Some(JungleTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#powered: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 7111 {
            return Some(JungleTrapdoor {
                r#powered: false,
                r#facing: Facing::North,
                r#open: false,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 7150 {
            return Some(JungleTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7159 {
            return Some(JungleTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 7156 {
            return Some(JungleTrapdoor {
                r#powered: false,
                r#half: Half::Top,
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 7165 {
            return Some(JungleTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#powered: true,
            });
        }
        if state_id == 7126 {
            return Some(JungleTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#open: false,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 7122 {
            return Some(JungleTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 7131 {
            return Some(JungleTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7119 {
            return Some(JungleTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 7164 {
            return Some(JungleTrapdoor {
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7151 {
            return Some(JungleTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#open: false,
                r#half: Half::Bottom,
                r#facing: Facing::West,
            });
        }
        if state_id == 7125 {
            return Some(JungleTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#powered: true,
                r#facing: Facing::South,
                r#half: Half::Top,
            });
        }
        if state_id == 7121 {
            return Some(JungleTrapdoor {
                r#powered: true,
                r#open: true,
                r#facing: Facing::South,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 7166 {
            return Some(JungleTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Bottom,
            });
        }
        return None;
    }
}

