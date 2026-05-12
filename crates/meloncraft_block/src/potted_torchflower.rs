use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedTorchflower {
}


impl BlockState for PottedTorchflower {
    fn to_id(self) -> i32 {
        return 10429;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10429 {
            return Some(PottedTorchflower {
            });
        }
        return None;
    }
}

