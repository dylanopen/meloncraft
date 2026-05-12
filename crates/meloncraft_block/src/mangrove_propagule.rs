use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangrovePropagule {
    pub waterlogged: bool,
    pub hanging: bool,
    pub age: i32,
    pub stage: i32,
}


impl BlockState for MangrovePropagule {
    fn to_id(self) -> i32 {
        if block_state.r#age == 0 && block_state.r#hanging == false && block_state.r#stage == 1 && block_state.r#waterlogged == false { return 52; }
        if block_state.r#age == 0 && block_state.r#stage == 1 && block_state.r#waterlogged == true && block_state.r#hanging == false { return 51; }
        if block_state.r#hanging == true && block_state.r#stage == 0 && block_state.r#age == 1 && block_state.r#waterlogged == false { return 54; }
        if block_state.r#hanging == true && block_state.r#stage == 1 && block_state.r#waterlogged == false && block_state.r#age == 2 { return 64; }
        if block_state.r#stage == 0 && block_state.r#waterlogged == false && block_state.r#age == 4 && block_state.r#hanging == false { return 82; }
        if block_state.r#hanging == true && block_state.r#age == 4 && block_state.r#stage == 1 && block_state.r#waterlogged == true { return 79; }
        if block_state.r#age == 3 && block_state.r#stage == 1 && block_state.r#waterlogged == true && block_state.r#hanging == false { return 75; }
        if block_state.r#age == 1 && block_state.r#hanging == false && block_state.r#waterlogged == true && block_state.r#stage == 0 { return 57; }
        if block_state.r#hanging == true && block_state.r#stage == 1 && block_state.r#age == 2 && block_state.r#waterlogged == true { return 63; }
        if block_state.r#age == 1 && block_state.r#waterlogged == false && block_state.r#hanging == false && block_state.r#stage == 1 { return 60; }
        if block_state.r#hanging == true && block_state.r#age == 3 && block_state.r#waterlogged == false && block_state.r#stage == 1 { return 72; }
        if block_state.r#hanging == false && block_state.r#waterlogged == true && block_state.r#age == 1 && block_state.r#stage == 1 { return 59; }
        if block_state.r#age == 0 && block_state.r#hanging == true && block_state.r#stage == 1 && block_state.r#waterlogged == true { return 47; }
        if block_state.r#age == 3 && block_state.r#hanging == true && block_state.r#waterlogged == false && block_state.r#stage == 0 { return 70; }
        if block_state.r#hanging == false && block_state.r#waterlogged == true && block_state.r#age == 0 && block_state.r#stage == 0 { return 49; }
        if block_state.r#age == 3 && block_state.r#waterlogged == false && block_state.r#hanging == false && block_state.r#stage == 0 { return 74; }
        if block_state.r#hanging == true && block_state.r#waterlogged == true && block_state.r#age == 0 && block_state.r#stage == 0 { return 45; }
        if block_state.r#age == 2 && block_state.r#stage == 0 && block_state.r#waterlogged == false && block_state.r#hanging == true { return 62; }
        if block_state.r#age == 2 && block_state.r#stage == 1 && block_state.r#waterlogged == false && block_state.r#hanging == false { return 68; }
        if block_state.r#stage == 1 && block_state.r#waterlogged == false && block_state.r#age == 3 && block_state.r#hanging == false { return 76; }
        if block_state.r#age == 4 && block_state.r#stage == 0 && block_state.r#waterlogged == true && block_state.r#hanging == false { return 81; }
        if block_state.r#waterlogged == true && block_state.r#age == 3 && block_state.r#hanging == true && block_state.r#stage == 1 { return 71; }
        if block_state.r#age == 4 && block_state.r#stage == 1 && block_state.r#hanging == true && block_state.r#waterlogged == false { return 80; }
        if block_state.r#hanging == false && block_state.r#age == 2 && block_state.r#waterlogged == true && block_state.r#stage == 1 { return 67; }
        if block_state.r#age == 4 && block_state.r#waterlogged == true && block_state.r#hanging == false && block_state.r#stage == 1 { return 83; }
        if block_state.r#age == 2 && block_state.r#hanging == false && block_state.r#waterlogged == false && block_state.r#stage == 0 { return 66; }
        if block_state.r#age == 0 && block_state.r#waterlogged == false && block_state.r#stage == 0 && block_state.r#hanging == false { return 50; }
        if block_state.r#stage == 0 && block_state.r#age == 2 && block_state.r#waterlogged == true && block_state.r#hanging == true { return 61; }
        if block_state.r#hanging == true && block_state.r#stage == 0 && block_state.r#waterlogged == true && block_state.r#age == 1 { return 53; }
        if block_state.r#stage == 0 && block_state.r#waterlogged == true && block_state.r#hanging == true && block_state.r#age == 3 { return 69; }
        if block_state.r#stage == 1 && block_state.r#hanging == true && block_state.r#age == 1 && block_state.r#waterlogged == true { return 55; }
        if block_state.r#age == 4 && block_state.r#hanging == false && block_state.r#stage == 1 && block_state.r#waterlogged == false { return 84; }
        if block_state.r#age == 1 && block_state.r#stage == 1 && block_state.r#waterlogged == false && block_state.r#hanging == true { return 56; }
        if block_state.r#stage == 1 && block_state.r#age == 0 && block_state.r#hanging == true && block_state.r#waterlogged == false { return 48; }
        if block_state.r#age == 0 && block_state.r#stage == 0 && block_state.r#waterlogged == false && block_state.r#hanging == true { return 46; }
        if block_state.r#age == 1 && block_state.r#waterlogged == false && block_state.r#stage == 0 && block_state.r#hanging == false { return 58; }
        if block_state.r#waterlogged == true && block_state.r#stage == 0 && block_state.r#hanging == false && block_state.r#age == 2 { return 65; }
        if block_state.r#hanging == false && block_state.r#age == 3 && block_state.r#waterlogged == true && block_state.r#stage == 0 { return 73; }
        if block_state.r#stage == 0 && block_state.r#hanging == true && block_state.r#waterlogged == true && block_state.r#age == 4 { return 77; }
        if block_state.r#waterlogged == false && block_state.r#hanging == true && block_state.r#stage == 0 && block_state.r#age == 4 { return 78; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 52 {
            return Some(MangrovePropagule {
                r#age: 0,
                r#hanging: false,
                r#stage: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 51 {
            return Some(MangrovePropagule {
                r#age: 0,
                r#stage: 1,
                r#waterlogged: true,
                r#hanging: false,
            });
        }
        if state_id == 54 {
            return Some(MangrovePropagule {
                r#hanging: true,
                r#stage: 0,
                r#age: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 64 {
            return Some(MangrovePropagule {
                r#hanging: true,
                r#stage: 1,
                r#waterlogged: false,
                r#age: 2,
            });
        }
        if state_id == 82 {
            return Some(MangrovePropagule {
                r#stage: 0,
                r#waterlogged: false,
                r#age: 4,
                r#hanging: false,
            });
        }
        if state_id == 79 {
            return Some(MangrovePropagule {
                r#hanging: true,
                r#age: 4,
                r#stage: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 75 {
            return Some(MangrovePropagule {
                r#age: 3,
                r#stage: 1,
                r#waterlogged: true,
                r#hanging: false,
            });
        }
        if state_id == 57 {
            return Some(MangrovePropagule {
                r#age: 1,
                r#hanging: false,
                r#waterlogged: true,
                r#stage: 0,
            });
        }
        if state_id == 63 {
            return Some(MangrovePropagule {
                r#hanging: true,
                r#stage: 1,
                r#age: 2,
                r#waterlogged: true,
            });
        }
        if state_id == 60 {
            return Some(MangrovePropagule {
                r#age: 1,
                r#waterlogged: false,
                r#hanging: false,
                r#stage: 1,
            });
        }
        if state_id == 72 {
            return Some(MangrovePropagule {
                r#hanging: true,
                r#age: 3,
                r#waterlogged: false,
                r#stage: 1,
            });
        }
        if state_id == 59 {
            return Some(MangrovePropagule {
                r#hanging: false,
                r#waterlogged: true,
                r#age: 1,
                r#stage: 1,
            });
        }
        if state_id == 47 {
            return Some(MangrovePropagule {
                r#age: 0,
                r#hanging: true,
                r#stage: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 70 {
            return Some(MangrovePropagule {
                r#age: 3,
                r#hanging: true,
                r#waterlogged: false,
                r#stage: 0,
            });
        }
        if state_id == 49 {
            return Some(MangrovePropagule {
                r#hanging: false,
                r#waterlogged: true,
                r#age: 0,
                r#stage: 0,
            });
        }
        if state_id == 74 {
            return Some(MangrovePropagule {
                r#age: 3,
                r#waterlogged: false,
                r#hanging: false,
                r#stage: 0,
            });
        }
        if state_id == 45 {
            return Some(MangrovePropagule {
                r#hanging: true,
                r#waterlogged: true,
                r#age: 0,
                r#stage: 0,
            });
        }
        if state_id == 62 {
            return Some(MangrovePropagule {
                r#age: 2,
                r#stage: 0,
                r#waterlogged: false,
                r#hanging: true,
            });
        }
        if state_id == 68 {
            return Some(MangrovePropagule {
                r#age: 2,
                r#stage: 1,
                r#waterlogged: false,
                r#hanging: false,
            });
        }
        if state_id == 76 {
            return Some(MangrovePropagule {
                r#stage: 1,
                r#waterlogged: false,
                r#age: 3,
                r#hanging: false,
            });
        }
        if state_id == 81 {
            return Some(MangrovePropagule {
                r#age: 4,
                r#stage: 0,
                r#waterlogged: true,
                r#hanging: false,
            });
        }
        if state_id == 71 {
            return Some(MangrovePropagule {
                r#waterlogged: true,
                r#age: 3,
                r#hanging: true,
                r#stage: 1,
            });
        }
        if state_id == 80 {
            return Some(MangrovePropagule {
                r#age: 4,
                r#stage: 1,
                r#hanging: true,
                r#waterlogged: false,
            });
        }
        if state_id == 67 {
            return Some(MangrovePropagule {
                r#hanging: false,
                r#age: 2,
                r#waterlogged: true,
                r#stage: 1,
            });
        }
        if state_id == 83 {
            return Some(MangrovePropagule {
                r#age: 4,
                r#waterlogged: true,
                r#hanging: false,
                r#stage: 1,
            });
        }
        if state_id == 66 {
            return Some(MangrovePropagule {
                r#age: 2,
                r#hanging: false,
                r#waterlogged: false,
                r#stage: 0,
            });
        }
        if state_id == 50 {
            return Some(MangrovePropagule {
                r#age: 0,
                r#waterlogged: false,
                r#stage: 0,
                r#hanging: false,
            });
        }
        if state_id == 61 {
            return Some(MangrovePropagule {
                r#stage: 0,
                r#age: 2,
                r#waterlogged: true,
                r#hanging: true,
            });
        }
        if state_id == 53 {
            return Some(MangrovePropagule {
                r#hanging: true,
                r#stage: 0,
                r#waterlogged: true,
                r#age: 1,
            });
        }
        if state_id == 69 {
            return Some(MangrovePropagule {
                r#stage: 0,
                r#waterlogged: true,
                r#hanging: true,
                r#age: 3,
            });
        }
        if state_id == 55 {
            return Some(MangrovePropagule {
                r#stage: 1,
                r#hanging: true,
                r#age: 1,
                r#waterlogged: true,
            });
        }
        if state_id == 84 {
            return Some(MangrovePropagule {
                r#age: 4,
                r#hanging: false,
                r#stage: 1,
                r#waterlogged: false,
            });
        }
        if state_id == 56 {
            return Some(MangrovePropagule {
                r#age: 1,
                r#stage: 1,
                r#waterlogged: false,
                r#hanging: true,
            });
        }
        if state_id == 48 {
            return Some(MangrovePropagule {
                r#stage: 1,
                r#age: 0,
                r#hanging: true,
                r#waterlogged: false,
            });
        }
        if state_id == 46 {
            return Some(MangrovePropagule {
                r#age: 0,
                r#stage: 0,
                r#waterlogged: false,
                r#hanging: true,
            });
        }
        if state_id == 58 {
            return Some(MangrovePropagule {
                r#age: 1,
                r#waterlogged: false,
                r#stage: 0,
                r#hanging: false,
            });
        }
        if state_id == 65 {
            return Some(MangrovePropagule {
                r#waterlogged: true,
                r#stage: 0,
                r#hanging: false,
                r#age: 2,
            });
        }
        if state_id == 73 {
            return Some(MangrovePropagule {
                r#hanging: false,
                r#age: 3,
                r#waterlogged: true,
                r#stage: 0,
            });
        }
        if state_id == 77 {
            return Some(MangrovePropagule {
                r#stage: 0,
                r#hanging: true,
                r#waterlogged: true,
                r#age: 4,
            });
        }
        if state_id == 78 {
            return Some(MangrovePropagule {
                r#waterlogged: false,
                r#hanging: true,
                r#stage: 0,
                r#age: 4,
            });
        }
        return None;
    }
}

