use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Torch {
}


impl BlockState for Torch {
    fn to_id(&self) -> i32 {
        return 3169;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 3169 {
            return Some(Torch {
            });
        }
        return None;
    }
}

