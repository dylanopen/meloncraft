use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightGrayConcretePowder {
}


impl BlockState for LightGrayConcretePowder {
    fn to_id(self) -> i32 {
        return 14852;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14852 {
            return Some(LightGrayConcretePowder {
            });
        }
        return None;
    }
}

