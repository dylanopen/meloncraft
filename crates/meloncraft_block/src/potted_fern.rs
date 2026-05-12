use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedFern {
}


impl BlockState for PottedFern {
    fn to_id(self) -> i32 {
        return 10439;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10439 {
            return Some(PottedFern {
            });
        }
        return None;
    }
}

