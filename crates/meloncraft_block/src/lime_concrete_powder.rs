use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LimeConcretePowder {
}


impl BlockState for LimeConcretePowder {
    fn to_id(self) -> i32 {
        return 14849;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14849 {
            return Some(LimeConcretePowder {
            });
        }
        return None;
    }
}

