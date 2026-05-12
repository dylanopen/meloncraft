use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadBrainCoralFan {
    pub waterlogged: bool,
}


impl BlockState for DeadBrainCoralFan {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false { return 14968; }
        if block_state.r#waterlogged == true { return 14967; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14968 {
            return Some(DeadBrainCoralFan {
                r#waterlogged: false,
            });
        }
        if state_id == 14967 {
            return Some(DeadBrainCoralFan {
                r#waterlogged: true,
            });
        }
        return None;
    }
}

