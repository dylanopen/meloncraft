use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedRoots {
}


impl BlockState for WarpedRoots {
    fn to_id(self) -> i32 {
        return 20758;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20758 {
            return Some(WarpedRoots {
            });
        }
        return None;
    }
}

