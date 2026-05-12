use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedstoneOre {
    pub lit: bool,
}


impl BlockState for RedstoneOre {
    fn to_id(self) -> i32 {
        if block_state.r#lit == true { return 6680; }
        if block_state.r#lit == false { return 6681; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6680 {
            return Some(RedstoneOre {
                r#lit: true,
            });
        }
        if state_id == 6681 {
            return Some(RedstoneOre {
                r#lit: false,
            });
        }
        return None;
    }
}

