use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfestedStoneBricks {
}


impl BlockState for InfestedStoneBricks {
    fn to_id(self) -> i32 {
        return 7561;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7561 {
            return Some(InfestedStoneBricks {
            });
        }
        return None;
    }
}

