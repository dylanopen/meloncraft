use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LimeStainedGlass {
}


impl BlockState for LimeStainedGlass {
    fn to_id(self) -> i32 {
        return 6902;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6902 {
            return Some(LimeStainedGlass {
            });
        }
        return None;
    }
}

