use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MossCarpet {
}


impl BlockState for MossCarpet {
    fn to_id(self) -> i32 {
        return 27611;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27611 {
            return Some(MossCarpet {
            });
        }
        return None;
    }
}

