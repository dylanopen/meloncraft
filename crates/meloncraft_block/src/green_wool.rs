use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreenWool {
}


impl BlockState for GreenWool {
    fn to_id(self) -> i32 {
        return 2106;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2106 {
            return Some(GreenWool {
            });
        }
        return None;
    }
}

