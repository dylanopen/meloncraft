use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YellowConcretePowder {
}


impl BlockState for YellowConcretePowder {
    fn to_id(&self) -> i32 {
        return 14848;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14848 {
            return Some(YellowConcretePowder {
            });
        }
        return None;
    }
}

