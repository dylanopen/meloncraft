use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CryingObsidian {
}


impl BlockState for CryingObsidian {
    fn to_id(self) -> i32 {
        return 21618;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21618 {
            return Some(CryingObsidian {
            });
        }
        return None;
    }
}

