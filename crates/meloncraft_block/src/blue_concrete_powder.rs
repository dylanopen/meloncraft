use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlueConcretePowder {
}


impl BlockState for BlueConcretePowder {
    fn to_id(self) -> i32 {
        return 14855;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14855 {
            return Some(BlueConcretePowder {
            });
        }
        return None;
    }
}

