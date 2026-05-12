use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Air {
}


impl BlockState for Air {
    fn to_id(&self) -> i32 {
        return 0;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 0 {
            return Some(Air {
            });
        }
        return None;
    }
}

