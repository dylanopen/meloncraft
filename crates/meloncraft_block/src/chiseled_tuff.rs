use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChiseledTuff {
}


impl BlockState for ChiseledTuff {
    fn to_id(self) -> i32 {
        return 24072;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 24072 {
            return Some(ChiseledTuff {
            });
        }
        return None;
    }
}

