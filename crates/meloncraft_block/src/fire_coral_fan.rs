use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FireCoralFan {
    pub waterlogged: bool,
}


impl BlockState for FireCoralFan {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true { return 14981; }
        if block_state.r#waterlogged == false { return 14982; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14981 {
            return Some(FireCoralFan {
                r#waterlogged: true,
            });
        }
        if state_id == 14982 {
            return Some(FireCoralFan {
                r#waterlogged: false,
            });
        }
        return None;
    }
}

