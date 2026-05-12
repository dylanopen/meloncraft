use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrayConcretePowder {
}


impl BlockState for GrayConcretePowder {
    fn to_id(&self) -> i32 {
        return 14851;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14851 {
            return Some(GrayConcretePowder {
            });
        }
        return None;
    }
}

