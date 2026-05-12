use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedCopperBlock {
}


impl BlockState for WaxedCopperBlock {
    fn to_id(&self) -> i32 {
        return 25469;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25469 {
            return Some(WaxedCopperBlock {
            });
        }
        return None;
    }
}

