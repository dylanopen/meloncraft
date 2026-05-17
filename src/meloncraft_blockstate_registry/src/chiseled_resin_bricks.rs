use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChiseledResinBricks {
}


impl BlockState for ChiseledResinBricks {
    fn to_id(&self) -> i32 {
        return 9132;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9132 {
            return Some(ChiseledResinBricks {
            });
        }
        return None;
    }
}

