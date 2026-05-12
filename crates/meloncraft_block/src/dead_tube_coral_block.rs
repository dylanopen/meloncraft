use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadTubeCoralBlock {
}


impl BlockState for DeadTubeCoralBlock {
    fn to_id(&self) -> i32 {
        return 14935;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14935 {
            return Some(DeadTubeCoralBlock {
            });
        }
        return None;
    }
}

