use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FireCoralBlock {
}


impl BlockState for FireCoralBlock {
    fn to_id(self) -> i32 {
        return 14943;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14943 {
            return Some(FireCoralBlock {
            });
        }
        return None;
    }
}

