use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedStainedGlass {
}


impl BlockState for RedStainedGlass {
    fn to_id(self) -> i32 {
        return 6911;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6911 {
            return Some(RedStainedGlass {
            });
        }
        return None;
    }
}

