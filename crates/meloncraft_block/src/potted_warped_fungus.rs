use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedWarpedFungus {
}


impl BlockState for PottedWarpedFungus {
    fn to_id(self) -> i32 {
        return 21625;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21625 {
            return Some(PottedWarpedFungus {
            });
        }
        return None;
    }
}

