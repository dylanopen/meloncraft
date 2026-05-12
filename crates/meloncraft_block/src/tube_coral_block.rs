use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TubeCoralBlock {
}


impl BlockState for TubeCoralBlock {
    fn to_id(self) -> i32 {
        return 14940;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14940 {
            return Some(TubeCoralBlock {
            });
        }
        return None;
    }
}

