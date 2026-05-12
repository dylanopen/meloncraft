use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadHornCoralFan {
    pub waterlogged: bool,
}


impl BlockState for DeadHornCoralFan {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false { return 14974; }
        if block_state.r#waterlogged == true { return 14973; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14974 {
            return Some(DeadHornCoralFan {
                r#waterlogged: false,
            });
        }
        if state_id == 14973 {
            return Some(DeadHornCoralFan {
                r#waterlogged: true,
            });
        }
        return None;
    }
}

