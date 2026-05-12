use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreenConcretePowder {
}


impl BlockState for GreenConcretePowder {
    fn to_id(&self) -> i32 {
        return 14857;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14857 {
            return Some(GreenConcretePowder {
            });
        }
        return None;
    }
}

