use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchTrapdoor {
    pub r#half: Half,
    pub powered: bool,
    pub r#facing: Facing,
    pub open: bool,
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

impl BlockState for BirchTrapdoor {
    fn to_id(self) -> i32 {
        if block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#open == false { return 7101; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::North { return 7041; }
        if block_state.r#half == Half::Top && block_state.r#facing == Facing::West && block_state.r#open == false && block_state.r#powered == false && block_state.r#waterlogged == true { return 7079; }
        if block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#waterlogged == true { return 7047; }
        if block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true && block_state.r#powered == false { return 7055; }
        if block_state.r#facing == Facing::East && block_state.r#waterlogged == true && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#open == true { return 7091; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 7071; }
        if block_state.r#facing == Facing::North && block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 7054; }
        if block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::North && block_state.r#powered == false { return 7044; }
        if block_state.r#waterlogged == false && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#powered == false { return 7088; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#open == false { return 7093; }
        if block_state.r#powered == false && block_state.r#half == Half::Bottom && block_state.r#waterlogged == false && block_state.r#facing == Facing::North && block_state.r#open == true { return 7052; }
        if block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#facing == Facing::North && block_state.r#waterlogged == false { return 7048; }
        if block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 7099; }
        if block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#half == Half::Top { return 7063; }
        if block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#waterlogged == true { return 7089; }
        if block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#facing == Facing::West { return 7086; }
        if block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::South { return 7066; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == true && block_state.r#waterlogged == true { return 7065; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#facing == Facing::South && block_state.r#open == true { return 7060; }
        if block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#waterlogged == false { return 7042; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#open == false { return 7056; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#powered == false { return 7059; }
        if block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#half == Half::Top { return 7080; }
        if block_state.r#open == true && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top { return 7075; }
        if block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#powered == false { return 7096; }
        if block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#facing == Facing::West && block_state.r#open == true { return 7084; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#open == true && block_state.r#powered == true { return 7058; }
        if block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#facing == Facing::East && block_state.r#powered == false && block_state.r#waterlogged == false { return 7100; }
        if block_state.r#facing == Facing::South && block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#open == false && block_state.r#half == Half::Bottom { return 7069; }
        if block_state.r#powered == true && block_state.r#waterlogged == true && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#facing == Facing::West { return 7085; }
        if block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#facing == Facing::East && block_state.r#open == false && block_state.r#half == Half::Bottom { return 7102; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#open == false && block_state.r#waterlogged == false { return 7094; }
        if block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 7049; }
        if block_state.r#powered == false && block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom { return 7068; }
        if block_state.r#half == Half::Top && block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#facing == Facing::East && block_state.r#powered == true { return 7090; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == true && block_state.r#waterlogged == false { return 7050; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#open == false { return 7062; }
        if block_state.r#powered == true && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#facing == Facing::North && block_state.r#waterlogged == true { return 7053; }
        if block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#powered == false { return 7043; }
        if block_state.r#waterlogged == false && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::South { return 7072; }
        if block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#powered == true && block_state.r#waterlogged == false { return 7078; }
        if block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#powered == true { return 7074; }
        if block_state.r#open == false && block_state.r#half == Half::Top && block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::East { return 7095; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == false { return 7103; }
        if block_state.r#facing == Facing::East && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#waterlogged == false && block_state.r#open == false { return 7104; }
        if block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#facing == Facing::West { return 7076; }
        if block_state.r#powered == false && block_state.r#open == false && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#waterlogged == true { return 7087; }
        if block_state.r#facing == Facing::South && block_state.r#powered == false && block_state.r#half == Half::Top && block_state.r#open == false && block_state.r#waterlogged == false { return 7064; }
        if block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#open == true && block_state.r#waterlogged == true { return 7083; }
        if block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#powered == false && block_state.r#open == true && block_state.r#facing == Facing::East { return 7092; }
        if block_state.r#waterlogged == true && block_state.r#open == true && block_state.r#half == Half::Bottom && block_state.r#powered == true && block_state.r#facing == Facing::East { return 7097; }
        if block_state.r#waterlogged == true && block_state.r#open == false && block_state.r#facing == Facing::South && block_state.r#half == Half::Top && block_state.r#powered == true { return 7061; }
        if block_state.r#waterlogged == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Bottom && block_state.r#open == true && block_state.r#powered == true { return 7081; }
        if block_state.r#facing == Facing::East && block_state.r#powered == true && block_state.r#open == true && block_state.r#waterlogged == false && block_state.r#half == Half::Bottom { return 7098; }
        if block_state.r#facing == Facing::North && block_state.r#powered == true && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#half == Half::Top { return 7045; }
        if block_state.r#open == true && block_state.r#half == Half::Top && block_state.r#facing == Facing::South && block_state.r#powered == true && block_state.r#waterlogged == true { return 7057; }
        if block_state.r#waterlogged == true && block_state.r#powered == true && block_state.r#open == true && block_state.r#facing == Facing::West && block_state.r#half == Half::Top { return 7073; }
        if block_state.r#half == Half::Top && block_state.r#powered == true && block_state.r#open == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::West { return 7077; }
        if block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#open == false && block_state.r#powered == true && block_state.r#waterlogged == false { return 7070; }
        if block_state.r#facing == Facing::North && block_state.r#half == Half::Bottom && block_state.r#powered == false && block_state.r#open == true && block_state.r#waterlogged == true { return 7051; }
        if block_state.r#half == Half::Bottom && block_state.r#facing == Facing::West && block_state.r#waterlogged == false && block_state.r#powered == true && block_state.r#open == true { return 7082; }
        if block_state.r#powered == true && block_state.r#facing == Facing::North && block_state.r#half == Half::Top && block_state.r#waterlogged == false && block_state.r#open == false { return 7046; }
        if block_state.r#powered == false && block_state.r#waterlogged == true && block_state.r#facing == Facing::South && block_state.r#half == Half::Bottom && block_state.r#open == true { return 7067; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7101 {
            return Some(BirchTrapdoor {
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::East,
                r#open: false,
            });
        }
        if state_id == 7041 {
            return Some(BirchTrapdoor {
                r#waterlogged: true,
                r#powered: true,
                r#open: true,
                r#half: Half::Top,
                r#facing: Facing::North,
            });
        }
        if state_id == 7079 {
            return Some(BirchTrapdoor {
                r#half: Half::Top,
                r#facing: Facing::West,
                r#open: false,
                r#powered: false,
                r#waterlogged: true,
            });
        }
        if state_id == 7047 {
            return Some(BirchTrapdoor {
                r#open: false,
                r#facing: Facing::North,
                r#powered: false,
                r#half: Half::Top,
                r#waterlogged: true,
            });
        }
        if state_id == 7055 {
            return Some(BirchTrapdoor {
                r#open: false,
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#waterlogged: true,
                r#powered: false,
            });
        }
        if state_id == 7091 {
            return Some(BirchTrapdoor {
                r#facing: Facing::East,
                r#waterlogged: true,
                r#half: Half::Top,
                r#powered: false,
                r#open: true,
            });
        }
        if state_id == 7071 {
            return Some(BirchTrapdoor {
                r#powered: false,
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 7054 {
            return Some(BirchTrapdoor {
                r#facing: Facing::North,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7044 {
            return Some(BirchTrapdoor {
                r#open: true,
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::North,
                r#powered: false,
            });
        }
        if state_id == 7088 {
            return Some(BirchTrapdoor {
                r#waterlogged: false,
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#powered: false,
            });
        }
        if state_id == 7093 {
            return Some(BirchTrapdoor {
                r#waterlogged: true,
                r#powered: true,
                r#facing: Facing::East,
                r#half: Half::Top,
                r#open: false,
            });
        }
        if state_id == 7052 {
            return Some(BirchTrapdoor {
                r#powered: false,
                r#half: Half::Bottom,
                r#waterlogged: false,
                r#facing: Facing::North,
                r#open: true,
            });
        }
        if state_id == 7048 {
            return Some(BirchTrapdoor {
                r#open: false,
                r#half: Half::Top,
                r#powered: false,
                r#facing: Facing::North,
                r#waterlogged: false,
            });
        }
        if state_id == 7099 {
            return Some(BirchTrapdoor {
                r#facing: Facing::East,
                r#powered: false,
                r#open: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 7063 {
            return Some(BirchTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7089 {
            return Some(BirchTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#facing: Facing::East,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7086 {
            return Some(BirchTrapdoor {
                r#powered: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 7066 {
            return Some(BirchTrapdoor {
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
                r#waterlogged: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 7065 {
            return Some(BirchTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7060 {
            return Some(BirchTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#powered: false,
                r#facing: Facing::South,
                r#open: true,
            });
        }
        if state_id == 7042 {
            return Some(BirchTrapdoor {
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7056 {
            return Some(BirchTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 7059 {
            return Some(BirchTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#open: true,
                r#powered: false,
            });
        }
        if state_id == 7080 {
            return Some(BirchTrapdoor {
                r#open: false,
                r#facing: Facing::West,
                r#powered: false,
                r#waterlogged: false,
                r#half: Half::Top,
            });
        }
        if state_id == 7075 {
            return Some(BirchTrapdoor {
                r#open: true,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 7096 {
            return Some(BirchTrapdoor {
                r#facing: Facing::East,
                r#open: false,
                r#waterlogged: false,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 7084 {
            return Some(BirchTrapdoor {
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
                r#facing: Facing::West,
                r#open: true,
            });
        }
        if state_id == 7058 {
            return Some(BirchTrapdoor {
                r#waterlogged: false,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 7100 {
            return Some(BirchTrapdoor {
                r#open: true,
                r#half: Half::Bottom,
                r#facing: Facing::East,
                r#powered: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7069 {
            return Some(BirchTrapdoor {
                r#facing: Facing::South,
                r#waterlogged: true,
                r#powered: true,
                r#open: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7085 {
            return Some(BirchTrapdoor {
                r#powered: true,
                r#waterlogged: true,
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 7102 {
            return Some(BirchTrapdoor {
                r#waterlogged: false,
                r#powered: true,
                r#facing: Facing::East,
                r#open: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7094 {
            return Some(BirchTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Top,
                r#powered: true,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7049 {
            return Some(BirchTrapdoor {
                r#open: true,
                r#facing: Facing::North,
                r#powered: true,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 7068 {
            return Some(BirchTrapdoor {
                r#powered: false,
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::South,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7090 {
            return Some(BirchTrapdoor {
                r#half: Half::Top,
                r#open: true,
                r#waterlogged: false,
                r#facing: Facing::East,
                r#powered: true,
            });
        }
        if state_id == 7050 {
            return Some(BirchTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7062 {
            return Some(BirchTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Top,
                r#waterlogged: false,
                r#powered: true,
                r#open: false,
            });
        }
        if state_id == 7053 {
            return Some(BirchTrapdoor {
                r#powered: true,
                r#half: Half::Bottom,
                r#open: false,
                r#facing: Facing::North,
                r#waterlogged: true,
            });
        }
        if state_id == 7043 {
            return Some(BirchTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#powered: false,
            });
        }
        if state_id == 7072 {
            return Some(BirchTrapdoor {
                r#waterlogged: false,
                r#half: Half::Bottom,
                r#powered: false,
                r#open: false,
                r#facing: Facing::South,
            });
        }
        if state_id == 7078 {
            return Some(BirchTrapdoor {
                r#half: Half::Top,
                r#open: false,
                r#facing: Facing::West,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7074 {
            return Some(BirchTrapdoor {
                r#open: true,
                r#facing: Facing::West,
                r#half: Half::Top,
                r#waterlogged: false,
                r#powered: true,
            });
        }
        if state_id == 7095 {
            return Some(BirchTrapdoor {
                r#open: false,
                r#half: Half::Top,
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 7103 {
            return Some(BirchTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: false,
            });
        }
        if state_id == 7104 {
            return Some(BirchTrapdoor {
                r#facing: Facing::East,
                r#half: Half::Bottom,
                r#powered: false,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 7076 {
            return Some(BirchTrapdoor {
                r#open: true,
                r#half: Half::Top,
                r#waterlogged: false,
                r#powered: false,
                r#facing: Facing::West,
            });
        }
        if state_id == 7087 {
            return Some(BirchTrapdoor {
                r#powered: false,
                r#open: false,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#waterlogged: true,
            });
        }
        if state_id == 7064 {
            return Some(BirchTrapdoor {
                r#facing: Facing::South,
                r#powered: false,
                r#half: Half::Top,
                r#open: false,
                r#waterlogged: false,
            });
        }
        if state_id == 7083 {
            return Some(BirchTrapdoor {
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#powered: false,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7092 {
            return Some(BirchTrapdoor {
                r#half: Half::Top,
                r#waterlogged: false,
                r#powered: false,
                r#open: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 7097 {
            return Some(BirchTrapdoor {
                r#waterlogged: true,
                r#open: true,
                r#half: Half::Bottom,
                r#powered: true,
                r#facing: Facing::East,
            });
        }
        if state_id == 7061 {
            return Some(BirchTrapdoor {
                r#waterlogged: true,
                r#open: false,
                r#facing: Facing::South,
                r#half: Half::Top,
                r#powered: true,
            });
        }
        if state_id == 7081 {
            return Some(BirchTrapdoor {
                r#waterlogged: true,
                r#facing: Facing::West,
                r#half: Half::Bottom,
                r#open: true,
                r#powered: true,
            });
        }
        if state_id == 7098 {
            return Some(BirchTrapdoor {
                r#facing: Facing::East,
                r#powered: true,
                r#open: true,
                r#waterlogged: false,
                r#half: Half::Bottom,
            });
        }
        if state_id == 7045 {
            return Some(BirchTrapdoor {
                r#facing: Facing::North,
                r#powered: true,
                r#open: false,
                r#waterlogged: true,
                r#half: Half::Top,
            });
        }
        if state_id == 7057 {
            return Some(BirchTrapdoor {
                r#open: true,
                r#half: Half::Top,
                r#facing: Facing::South,
                r#powered: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7073 {
            return Some(BirchTrapdoor {
                r#waterlogged: true,
                r#powered: true,
                r#open: true,
                r#facing: Facing::West,
                r#half: Half::Top,
            });
        }
        if state_id == 7077 {
            return Some(BirchTrapdoor {
                r#half: Half::Top,
                r#powered: true,
                r#open: false,
                r#waterlogged: true,
                r#facing: Facing::West,
            });
        }
        if state_id == 7070 {
            return Some(BirchTrapdoor {
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: false,
                r#powered: true,
                r#waterlogged: false,
            });
        }
        if state_id == 7051 {
            return Some(BirchTrapdoor {
                r#facing: Facing::North,
                r#half: Half::Bottom,
                r#powered: false,
                r#open: true,
                r#waterlogged: true,
            });
        }
        if state_id == 7082 {
            return Some(BirchTrapdoor {
                r#half: Half::Bottom,
                r#facing: Facing::West,
                r#waterlogged: false,
                r#powered: true,
                r#open: true,
            });
        }
        if state_id == 7046 {
            return Some(BirchTrapdoor {
                r#powered: true,
                r#facing: Facing::North,
                r#half: Half::Top,
                r#waterlogged: false,
                r#open: false,
            });
        }
        if state_id == 7067 {
            return Some(BirchTrapdoor {
                r#powered: false,
                r#waterlogged: true,
                r#facing: Facing::South,
                r#half: Half::Bottom,
                r#open: true,
            });
        }
        return None;
    }
}

