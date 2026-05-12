use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedDeadBush {
}


impl BlockState for PottedDeadBush {
    fn to_id(&self) -> i32 {
        return 10455;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10455 {
            return Some(PottedDeadBush {
            });
        }
        return None;
    }
}

