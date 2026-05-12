use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhiteStainedGlass {
}


impl BlockState for WhiteStainedGlass {
    fn to_id(&self) -> i32 {
        return 6897;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6897 {
            return Some(WhiteStainedGlass {
            });
        }
        return None;
    }
}

