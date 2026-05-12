use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CyanWool {
}


impl BlockState for CyanWool {
    fn to_id(self) -> i32 {
        return 2102;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2102 {
            return Some(CyanWool {
            });
        }
        return None;
    }
}

