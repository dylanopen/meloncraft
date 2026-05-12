use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeepslateRedstoneOre {
    pub lit: bool,
}


impl BlockState for DeepslateRedstoneOre {
    fn to_id(self) -> i32 {
        if block_state.r#lit == true { return 6682; }
        if block_state.r#lit == false { return 6683; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6682 {
            return Some(DeepslateRedstoneOre {
                r#lit: true,
            });
        }
        if state_id == 6683 {
            return Some(DeepslateRedstoneOre {
                r#lit: false,
            });
        }
        return None;
    }
}

