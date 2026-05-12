use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrainCoral {
    pub waterlogged: bool,
}


impl BlockState for BrainCoral {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false { return 14958; }
        if block_state.r#waterlogged == true { return 14957; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14958 {
            return Some(BrainCoral {
                r#waterlogged: false,
            });
        }
        if state_id == 14957 {
            return Some(BrainCoral {
                r#waterlogged: true,
            });
        }
        return None;
    }
}

