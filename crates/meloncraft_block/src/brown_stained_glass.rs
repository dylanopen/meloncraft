use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrownStainedGlass {
}


impl BlockState for BrownStainedGlass {
    fn to_id(self) -> i32 {
        return 6909;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6909 {
            return Some(BrownStainedGlass {
            });
        }
        return None;
    }
}

