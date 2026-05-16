use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmoothSandstone {
}


impl BlockState for SmoothSandstone {
    fn to_id(&self) -> i32 {
        return 13279;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13279 {
            return Some(SmoothSandstone {
            });
        }
        return None;
    }
}

