use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedDandelion {
}


impl BlockState for PottedDandelion {
    fn to_id(self) -> i32 {
        return 10440;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10440 {
            return Some(PottedDandelion {
            });
        }
        return None;
    }
}

