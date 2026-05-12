use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightGrayWool {
}


impl BlockState for LightGrayWool {
    fn to_id(self) -> i32 {
        return 2101;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2101 {
            return Some(LightGrayWool {
            });
        }
        return None;
    }
}

