use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchTrapdoor {
    pub open: bool,
    pub r#half: Half,
    pub waterlogged: bool,
    pub powered: bool,
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

impl BlockState for BirchTrapdoor {
    fn to_id(&self) -> i32 {
        if self.r#open == false && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#powered == false && self.r#half == Half::Bottom { return 7055; }
        if self.r#half == Half::Top && self.r#open == false && self.r#facing == Facing::West && self.r#waterlogged == false && self.r#powered == false { return 7080; }
        if self.r#open == true && self.r#half == Half::Bottom && self.r#powered == false && self.r#waterlogged == false && self.r#facing == Facing::North { return 7052; }
        if self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::North && self.r#open == true && self.r#half == Half::Bottom { return 7050; }
        if self.r#half == Half::Bottom && self.r#powered == true && self.r#waterlogged == false && self.r#facing == Facing::West && self.r#open == true { return 7082; }
        if self.r#facing == Facing::East && self.r#half == Half::Top && self.r#waterlogged == true && self.r#open == true && self.r#powered == false { return 7091; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#open == true && self.r#powered == false { return 7068; }
        if self.r#open == true && self.r#powered == false && self.r#half == Half::Top && self.r#waterlogged == false && self.r#facing == Facing::East { return 7092; }
        if self.r#facing == Facing::South && self.r#open == true && self.r#powered == false && self.r#waterlogged == true && self.r#half == Half::Bottom { return 7067; }
        if self.r#open == false && self.r#powered == true && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#facing == Facing::East { return 7101; }
        if self.r#facing == Facing::East && self.r#open == false && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#powered == false { return 7104; }
        if self.r#waterlogged == true && self.r#facing == Facing::South && self.r#half == Half::Top && self.r#open == true && self.r#powered == true { return 7057; }
        if self.r#open == false && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#powered == false && self.r#facing == Facing::West { return 7087; }
        if self.r#half == Half::Top && self.r#facing == Facing::East && self.r#powered == true && self.r#open == false && self.r#waterlogged == false { return 7094; }
        if self.r#open == false && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#powered == false && self.r#facing == Facing::South { return 7071; }
        if self.r#half == Half::Top && self.r#powered == true && self.r#facing == Facing::South && self.r#waterlogged == false && self.r#open == false { return 7062; }
        if self.r#powered == false && self.r#half == Half::Top && self.r#facing == Facing::East && self.r#open == false && self.r#waterlogged == false { return 7096; }
        if self.r#facing == Facing::South && self.r#half == Half::Top && self.r#waterlogged == true && self.r#open == true && self.r#powered == false { return 7059; }
        if self.r#powered == true && self.r#facing == Facing::East && self.r#open == true && self.r#waterlogged == false && self.r#half == Half::Top { return 7090; }
        if self.r#half == Half::Bottom && self.r#open == false && self.r#waterlogged == false && self.r#facing == Facing::South && self.r#powered == false { return 7072; }
        if self.r#half == Half::Bottom && self.r#facing == Facing::East && self.r#open == false && self.r#powered == true && self.r#waterlogged == false { return 7102; }
        if self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#open == false && self.r#facing == Facing::South && self.r#powered == true { return 7070; }
        if self.r#open == false && self.r#half == Half::Top && self.r#waterlogged == false && self.r#powered == false && self.r#facing == Facing::South { return 7064; }
        if self.r#waterlogged == false && self.r#facing == Facing::West && self.r#open == false && self.r#half == Half::Top && self.r#powered == true { return 7078; }
        if self.r#open == true && self.r#half == Half::Top && self.r#powered == true && self.r#facing == Facing::South && self.r#waterlogged == false { return 7058; }
        if self.r#facing == Facing::North && self.r#waterlogged == false && self.r#powered == true && self.r#open == false && self.r#half == Half::Bottom { return 7054; }
        if self.r#facing == Facing::West && self.r#half == Half::Bottom && self.r#powered == true && self.r#open == false && self.r#waterlogged == false { return 7086; }
        if self.r#facing == Facing::West && self.r#waterlogged == true && self.r#powered == false && self.r#open == true && self.r#half == Half::Bottom { return 7083; }
        if self.r#facing == Facing::West && self.r#half == Half::Top && self.r#powered == true && self.r#waterlogged == false && self.r#open == true { return 7074; }
        if self.r#facing == Facing::South && self.r#open == false && self.r#half == Half::Top && self.r#waterlogged == true && self.r#powered == false { return 7063; }
        if self.r#facing == Facing::North && self.r#powered == true && self.r#half == Half::Top && self.r#waterlogged == true && self.r#open == true { return 7041; }
        if self.r#waterlogged == true && self.r#open == false && self.r#powered == true && self.r#half == Half::Bottom && self.r#facing == Facing::South { return 7069; }
        if self.r#waterlogged == true && self.r#facing == Facing::West && self.r#open == false && self.r#half == Half::Top && self.r#powered == true { return 7077; }
        if self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#open == true && self.r#facing == Facing::West && self.r#powered == false { return 7084; }
        if self.r#powered == true && self.r#facing == Facing::West && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#open == true { return 7081; }
        if self.r#half == Half::Bottom && self.r#open == true && self.r#powered == true && self.r#facing == Facing::East && self.r#waterlogged == false { return 7098; }
        if self.r#half == Half::Top && self.r#powered == false && self.r#facing == Facing::West && self.r#open == true && self.r#waterlogged == false { return 7076; }
        if self.r#facing == Facing::North && self.r#open == true && self.r#half == Half::Bottom && self.r#powered == false && self.r#waterlogged == true { return 7051; }
        if self.r#half == Half::Top && self.r#facing == Facing::South && self.r#open == false && self.r#powered == true && self.r#waterlogged == true { return 7061; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#powered == false && self.r#waterlogged == true && self.r#open == true { return 7043; }
        if self.r#half == Half::Top && self.r#powered == true && self.r#open == false && self.r#facing == Facing::East && self.r#waterlogged == true { return 7093; }
        if self.r#powered == true && self.r#waterlogged == true && self.r#facing == Facing::North && self.r#open == false && self.r#half == Half::Bottom { return 7053; }
        if self.r#half == Half::Top && self.r#powered == true && self.r#facing == Facing::North && self.r#waterlogged == false && self.r#open == false { return 7046; }
        if self.r#open == false && self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::West && self.r#powered == true { return 7085; }
        if self.r#open == false && self.r#half == Half::Top && self.r#powered == false && self.r#facing == Facing::East && self.r#waterlogged == true { return 7095; }
        if self.r#waterlogged == true && self.r#open == true && self.r#facing == Facing::West && self.r#powered == true && self.r#half == Half::Top { return 7073; }
        if self.r#powered == false && self.r#facing == Facing::North && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#open == false { return 7056; }
        if self.r#open == true && self.r#facing == Facing::North && self.r#powered == false && self.r#waterlogged == false && self.r#half == Half::Top { return 7044; }
        if self.r#waterlogged == true && self.r#open == false && self.r#facing == Facing::North && self.r#half == Half::Top && self.r#powered == false { return 7047; }
        if self.r#facing == Facing::North && self.r#half == Half::Top && self.r#open == true && self.r#waterlogged == false && self.r#powered == true { return 7042; }
        if self.r#open == true && self.r#facing == Facing::North && self.r#waterlogged == true && self.r#powered == true && self.r#half == Half::Bottom { return 7049; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#open == true && self.r#facing == Facing::West && self.r#powered == false { return 7075; }
        if self.r#open == true && self.r#waterlogged == true && self.r#facing == Facing::East && self.r#half == Half::Top && self.r#powered == true { return 7089; }
        if self.r#facing == Facing::East && self.r#open == true && self.r#half == Half::Bottom && self.r#powered == false && self.r#waterlogged == true { return 7099; }
        if self.r#waterlogged == false && self.r#powered == true && self.r#half == Half::Bottom && self.r#open == true && self.r#facing == Facing::South { return 7066; }
        if self.r#half == Half::Top && self.r#waterlogged == false && self.r#open == false && self.r#facing == Facing::North && self.r#powered == false { return 7048; }
        if self.r#facing == Facing::West && self.r#open == false && self.r#waterlogged == false && self.r#half == Half::Bottom && self.r#powered == false { return 7088; }
        if self.r#waterlogged == false && self.r#facing == Facing::South && self.r#powered == false && self.r#half == Half::Top && self.r#open == true { return 7060; }
        if self.r#open == false && self.r#half == Half::Bottom && self.r#powered == false && self.r#waterlogged == true && self.r#facing == Facing::East { return 7103; }
        if self.r#waterlogged == true && self.r#half == Half::Bottom && self.r#facing == Facing::South && self.r#open == true && self.r#powered == true { return 7065; }
        if self.r#half == Half::Top && self.r#waterlogged == true && self.r#powered == false && self.r#facing == Facing::West && self.r#open == false { return 7079; }
        if self.r#facing == Facing::East && self.r#powered == true && self.r#half == Half::Bottom && self.r#waterlogged == true && self.r#open == true { return 7097; }
        if self.r#open == true && self.r#half == Half::Bottom && self.r#waterlogged == false && self.r#facing == Facing::East && self.r#powered == false { return 7100; }
        if self.r#waterlogged == true && self.r#half == Half::Top && self.r#powered == true && self.r#facing == Facing::North && self.r#open == false { return 7045; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7055 {
            return Some(BirchTrapdoor {
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7080 {
            return Some(BirchTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: false,
            });
        }
        if state_id == 7052 {
            return Some(BirchTrapdoor {
                r#open: true,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::North,
            });
        }
        if state_id == 7050 {
            return Some(BirchTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#open: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7082 {
            return Some(BirchTrapdoor {
                r#half: Half::Bottom,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 7091 {
            return Some(BirchTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 7068 {
            return Some(BirchTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 7092 {
            return Some(BirchTrapdoor {
                r#open: true,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: false,
                r#facing: Facing::East,
            });
        }
        if state_id == 7067 {
            return Some(BirchTrapdoor {
                r#facing: Facing::South,
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7101 {
            return Some(BirchTrapdoor {
                r#open: false,
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 7104 {
            return Some(BirchTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 7057 {
            return Some(BirchTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 7087 {
            return Some(BirchTrapdoor {
                r#open: false,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 7094 {
            return Some(BirchTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::East,
                r#powered: true,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7071 {
            return Some(BirchTrapdoor {
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 7062 {
            return Some(BirchTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 7096 {
            return Some(BirchTrapdoor {
                r#powered: false,
                r#half: Half::Top,
                r#facing: Facing::East,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7059 {
            return Some(BirchTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 7090 {
            return Some(BirchTrapdoor {
                r#powered: true,
                r#facing: Facing::East,
                r#open: true,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7072 {
            return Some(BirchTrapdoor {
                r#half: Half::Bottom,
                r#open: false,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#powered: false,
            });
        }
        if state_id == 7102 {
            return Some(BirchTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7070 {
            return Some(BirchTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::South,
                r#powered: true,
            });
        }
        if state_id == 7064 {
            return Some(BirchTrapdoor {
                r#open: false,
                r#half: Half::Top,
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 7078 {
            return Some(BirchTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 7058 {
            return Some(BirchTrapdoor {
                r#open: true,
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::South,
                r#waterlogged: false,
            });
        }
        if state_id == 7054 {
            return Some(BirchTrapdoor {
                r#facing: Facing::North,
                r#waterlogged: false,
                r#powered: true,
                r#open: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7086 {
            return Some(BirchTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#powered: true,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7083 {
            return Some(BirchTrapdoor {
                r#facing: Facing::West,
                r#waterlogged: true,
                r#powered: false,
                r#open: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7074 {
            return Some(BirchTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: false,
                r#open: true,
            });
        }
        if state_id == 7063 {
            return Some(BirchTrapdoor {
                r#facing: Facing::South,
                r#open: false,
                r#half: Half::Top,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 7041 {
            return Some(BirchTrapdoor {
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 7069 {
            return Some(BirchTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#powered: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
            });
        }
        if state_id == 7077 {
            return Some(BirchTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#open: false,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 7084 {
            return Some(BirchTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: true,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 7081 {
            return Some(BirchTrapdoor {
                r#powered: true,
                r#facing: Facing::West,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        if state_id == 7098 {
            return Some(BirchTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
                r#facing: Facing::East,
                r#waterlogged: false,
            });
        }
        if state_id == 7076 {
            return Some(BirchTrapdoor {
                r#half: Half::Top,
                r#powered: false,
                r#facing: Facing::West,
                r#open: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7051 {
            return Some(BirchTrapdoor {
                r#facing: Facing::North,
                r#open: true,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7061 {
            return Some(BirchTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::South,
                r#open: false,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7043 {
            return Some(BirchTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 7093 {
            return Some(BirchTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#open: false,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 7053 {
            return Some(BirchTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#facing: Facing::North,
                r#open: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7046 {
            return Some(BirchTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::North,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 7085 {
            return Some(BirchTrapdoor {
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#powered: true,
            });
        }
        if state_id == 7095 {
            return Some(BirchTrapdoor {
                r#open: false,
                r#half: Half::Top,
                r#powered: false,
                r#facing: Facing::East,
                r#waterlogged: true,
            });
        }
        if state_id == 7073 {
            return Some(BirchTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::West,
                r#powered: true,
                r#half: Half::Top,
            });
        }
        if state_id == 7056 {
            return Some(BirchTrapdoor {
                r#powered: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 7044 {
            return Some(BirchTrapdoor {
                r#open: true,
                r#facing: Facing::North,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7047 {
            return Some(BirchTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 7042 {
            return Some(BirchTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Top,
                r#open: true,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 7049 {
            return Some(BirchTrapdoor {
                r#open: true,
                r#facing: Facing::North,
                r#waterlogged: true,
                r#powered: true,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7075 {
            return Some(BirchTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::West,
                r#powered: false,
            });
        }
        if state_id == 7089 {
            return Some(BirchTrapdoor {
                r#open: true,
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 7099 {
            return Some(BirchTrapdoor {
                r#facing: Facing::East,
                r#open: true,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7066 {
            return Some(BirchTrapdoor {
                r#waterlogged: false,
                r#powered: true,
                r#half: Half::Bottom,
                r#open: true,
                r#facing: Facing::South,
            });
        }
        if state_id == 7048 {
            return Some(BirchTrapdoor {
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 7088 {
            return Some(BirchTrapdoor {
                r#facing: Facing::West,
                r#open: false,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 7060 {
            return Some(BirchTrapdoor {
                r#waterlogged: false,
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Top,
                r#open: true,
            });
        }
        if state_id == 7103 {
            return Some(BirchTrapdoor {
                r#open: false,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 7065 {
            return Some(BirchTrapdoor {
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 7079 {
            return Some(BirchTrapdoor {
                r#half: Half::Top,
                r#waterlogged: true,
                r#powered: false,
                r#facing: Facing::West,
                r#open: false,
            });
        }
        if state_id == 7097 {
            return Some(BirchTrapdoor {
                r#facing: Facing::East,
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#open: true,
            });
        }
        if state_id == 7100 {
            return Some(BirchTrapdoor {
                r#open: true,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#powered: false,
            });
        }
        if state_id == 7045 {
            return Some(BirchTrapdoor {
                r#waterlogged: true,
                r#half: Half::Top,
                r#powered: true,
                r#facing: Facing::North,
                r#open: false,
            });
        }
        return None;
    }
}

