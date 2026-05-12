use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangroveRoots {
    pub waterlogged: bool,
}


impl BlockState for MangroveRoots {
    fn to_id(self) -> i32 {
        if block_state.r#waterlogged == false { return 164; }
        if block_state.r#waterlogged == true { return 163; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 164 {
            return Some(MangroveRoots {
                r#waterlogged: false,
            });
        }
        if state_id == 163 {
            return Some(MangroveRoots {
                r#waterlogged: true,
            });
        }
        return None;
    }
}

