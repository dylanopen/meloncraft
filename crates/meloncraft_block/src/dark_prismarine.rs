use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DarkPrismarine {
}


impl BlockState for DarkPrismarine {
    fn to_id(self) -> i32 {
        return 12431;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12431 {
            return Some(DarkPrismarine {
            });
        }
        return None;
    }
}

