use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TubeCoralFan {
    pub waterlogged: bool,
}


impl BlockState for TubeCoralFan {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == true { return 14975; }
        if block_state.r#waterlogged == false { return 14976; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14975 {
            return Some(TubeCoralFan {
                r#waterlogged: true,
            });
        }
        if state_id == 14976 {
            return Some(TubeCoralFan {
                r#waterlogged: false,
            });
        }
        return None;
    }
}

