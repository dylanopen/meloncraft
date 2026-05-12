use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Obsidian {
}


impl BlockState for Obsidian {
    fn to_id(self) -> i32 {
        return 3168;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 3168 {
            return Some(Obsidian {
            });
        }
        return None;
    }
}

