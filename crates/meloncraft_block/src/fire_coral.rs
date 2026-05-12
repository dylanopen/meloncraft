use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FireCoral {
    pub waterlogged: bool,
}


impl BlockState for FireCoral {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false { return 14962; }
        if block_state.r#waterlogged == true { return 14961; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14962 {
            return Some(FireCoral {
                r#waterlogged: false,
            });
        }
        if state_id == 14961 {
            return Some(FireCoral {
                r#waterlogged: true,
            });
        }
        return None;
    }
}

