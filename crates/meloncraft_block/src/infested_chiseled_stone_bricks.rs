use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfestedChiseledStoneBricks {
}


impl BlockState for InfestedChiseledStoneBricks {
    fn to_id(&self) -> i32 {
        return 7564;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7564 {
            return Some(InfestedChiseledStoneBricks {
            });
        }
        return None;
    }
}

