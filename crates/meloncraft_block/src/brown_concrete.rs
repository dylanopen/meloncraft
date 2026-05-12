use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrownConcrete {
}


impl BlockState for BrownConcrete {
    fn to_id(self) -> i32 {
        return 14840;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14840 {
            return Some(BrownConcrete {
            });
        }
        return None;
    }
}

