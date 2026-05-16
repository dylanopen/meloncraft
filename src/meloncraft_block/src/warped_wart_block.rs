use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedWartBlock {
}


impl BlockState for WarpedWartBlock {
    fn to_id(&self) -> i32 {
        return 20757;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20757 {
            return Some(WarpedWartBlock {
            });
        }
        return None;
    }
}

