use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedAzaleaBush {
}


impl BlockState for PottedAzaleaBush {
    fn to_id(&self) -> i32 {
        return 29378;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29378 {
            return Some(PottedAzaleaBush {
            });
        }
        return None;
    }
}

