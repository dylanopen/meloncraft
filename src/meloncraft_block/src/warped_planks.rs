use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedPlanks {
}


impl BlockState for WarpedPlanks {
    fn to_id(&self) -> i32 {
        return 20831;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20831 {
            return Some(WarpedPlanks {
            });
        }
        return None;
    }
}

