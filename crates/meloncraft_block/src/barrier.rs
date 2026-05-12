use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Barrier {
    pub waterlogged: bool,
}


impl BlockState for Barrier {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false { return 12332; }
        if block_state.r#waterlogged == true { return 12331; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12332 {
            return Some(Barrier {
                r#waterlogged: false,
            });
        }
        if state_id == 12331 {
            return Some(Barrier {
                r#waterlogged: true,
            });
        }
        return None;
    }
}

