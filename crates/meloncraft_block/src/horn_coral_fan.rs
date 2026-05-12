use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HornCoralFan {
    pub waterlogged: bool,
}


impl BlockState for HornCoralFan {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false { return 14984; }
        if block_state.r#waterlogged == true { return 14983; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14984 {
            return Some(HornCoralFan {
                r#waterlogged: false,
            });
        }
        if state_id == 14983 {
            return Some(HornCoralFan {
                r#waterlogged: true,
            });
        }
        return None;
    }
}

