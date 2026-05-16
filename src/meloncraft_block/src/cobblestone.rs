use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cobblestone {
}


impl BlockState for Cobblestone {
    fn to_id(&self) -> i32 {
        return 14;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14 {
            return Some(Cobblestone {
            });
        }
        return None;
    }
}

