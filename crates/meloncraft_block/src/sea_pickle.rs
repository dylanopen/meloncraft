use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeaPickle {
    pub pickles: i32,
    pub waterlogged: bool,
}


impl BlockState for SeaPickle {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false && block_state.r#pickles == 1 { return 15066; }
        if block_state.r#waterlogged == false && block_state.r#pickles == 2 { return 15068; }
        if block_state.r#waterlogged == true && block_state.r#pickles == 1 { return 15065; }
        if block_state.r#pickles == 4 && block_state.r#waterlogged == false { return 15072; }
        if block_state.r#pickles == 3 && block_state.r#waterlogged == false { return 15070; }
        if block_state.r#waterlogged == true && block_state.r#pickles == 4 { return 15071; }
        if block_state.r#pickles == 3 && block_state.r#waterlogged == true { return 15069; }
        if block_state.r#waterlogged == true && block_state.r#pickles == 2 { return 15067; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15066 {
            return Some(SeaPickle {
                r#waterlogged: false,
                r#pickles: 1,
            });
        }
        if state_id == 15068 {
            return Some(SeaPickle {
                r#waterlogged: false,
                r#pickles: 2,
            });
        }
        if state_id == 15065 {
            return Some(SeaPickle {
                r#waterlogged: true,
                r#pickles: 1,
            });
        }
        if state_id == 15072 {
            return Some(SeaPickle {
                r#pickles: 4,
                r#waterlogged: false,
            });
        }
        if state_id == 15070 {
            return Some(SeaPickle {
                r#pickles: 3,
                r#waterlogged: false,
            });
        }
        if state_id == 15071 {
            return Some(SeaPickle {
                r#waterlogged: true,
                r#pickles: 4,
            });
        }
        if state_id == 15069 {
            return Some(SeaPickle {
                r#pickles: 3,
                r#waterlogged: true,
            });
        }
        if state_id == 15067 {
            return Some(SeaPickle {
                r#waterlogged: true,
                r#pickles: 2,
            });
        }
        return None;
    }
}

