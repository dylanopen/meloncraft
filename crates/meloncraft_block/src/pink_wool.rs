use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinkWool {
}


impl BlockState for PinkWool {
    fn to_id(self) -> i32 {
        return 2099;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2099 {
            return Some(PinkWool {
            });
        }
        return None;
    }
}

