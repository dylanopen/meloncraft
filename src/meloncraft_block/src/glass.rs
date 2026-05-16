use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Glass {
}


impl BlockState for Glass {
    fn to_id(&self) -> i32 {
        return 562;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 562 {
            return Some(Glass {
            });
        }
        return None;
    }
}

