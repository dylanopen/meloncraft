use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeepslateTiles {
}


impl BlockState for DeepslateTiles {
    fn to_id(self) -> i32 {
        return 28546;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 28546 {
            return Some(DeepslateTiles {
            });
        }
        return None;
    }
}

