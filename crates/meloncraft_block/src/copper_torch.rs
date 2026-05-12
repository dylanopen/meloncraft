use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopperTorch {
}


impl BlockState for CopperTorch {
    fn to_id(self) -> i32 {
        return 6810;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6810 {
            return Some(CopperTorch {
            });
        }
        return None;
    }
}

