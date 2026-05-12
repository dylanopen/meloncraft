use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoulTorch {
}


impl BlockState for SoulTorch {
    fn to_id(&self) -> i32 {
        return 6805;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6805 {
            return Some(SoulTorch {
            });
        }
        return None;
    }
}

