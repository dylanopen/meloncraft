use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedstoneOre {
    pub lit: bool,
}


impl BlockState for RedstoneOre {
    fn to_id(&self) -> i32 {
        if self.r#lit == false { return 6681; }
        if self.r#lit == true { return 6680; }
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6681 {
            return Some(RedstoneOre {
                r#lit: false,
            });
        }
        if state_id == 6680 {
            return Some(RedstoneOre {
                r#lit: true,
            });
        }
        return None;
    }
}

