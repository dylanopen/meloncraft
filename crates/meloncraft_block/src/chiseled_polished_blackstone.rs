use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChiseledPolishedBlackstone {
}


impl BlockState for ChiseledPolishedBlackstone {
    fn to_id(&self) -> i32 {
        return 22043;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22043 {
            return Some(ChiseledPolishedBlackstone {
            });
        }
        return None;
    }
}

