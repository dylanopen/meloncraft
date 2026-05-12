use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfestedMossyStoneBricks {
}


impl BlockState for InfestedMossyStoneBricks {
    fn to_id(&self) -> i32 {
        return 7562;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7562 {
            return Some(InfestedMossyStoneBricks {
            });
        }
        return None;
    }
}

