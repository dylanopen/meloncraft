use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YellowStainedGlass {
}


impl BlockState for YellowStainedGlass {
    fn to_id(self) -> i32 {
        return 6901;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6901 {
            return Some(YellowStainedGlass {
            });
        }
        return None;
    }
}

