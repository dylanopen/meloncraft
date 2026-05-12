use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadTubeCoralFan {
    pub waterlogged: bool,
}


impl BlockState for DeadTubeCoralFan {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false { return 14966; }
        if block_state.r#waterlogged == true { return 14965; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14966 {
            return Some(DeadTubeCoralFan {
                r#waterlogged: false,
            });
        }
        if state_id == 14965 {
            return Some(DeadTubeCoralFan {
                r#waterlogged: true,
            });
        }
        return None;
    }
}

