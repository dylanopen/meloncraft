use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfestedCobblestone {
}


impl BlockState for InfestedCobblestone {
    fn to_id(self) -> i32 {
        return 7560;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7560 {
            return Some(InfestedCobblestone {
            });
        }
        return None;
    }
}

