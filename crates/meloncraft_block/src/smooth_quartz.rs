use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmoothQuartz {
}


impl BlockState for SmoothQuartz {
    fn to_id(&self) -> i32 {
        return 13280;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13280 {
            return Some(SmoothQuartz {
            });
        }
        return None;
    }
}

