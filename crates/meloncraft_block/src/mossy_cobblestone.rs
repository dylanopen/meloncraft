use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MossyCobblestone {
}


impl BlockState for MossyCobblestone {
    fn to_id(self) -> i32 {
        return 3167;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 3167 {
            return Some(MossyCobblestone {
            });
        }
        return None;
    }
}

