use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedFungus {
}


impl BlockState for WarpedFungus {
    fn to_id(self) -> i32 {
        return 20756;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20756 {
            return Some(WarpedFungus {
            });
        }
        return None;
    }
}

