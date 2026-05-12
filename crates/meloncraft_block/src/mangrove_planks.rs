use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MangrovePlanks {
}


impl BlockState for MangrovePlanks {
    fn to_id(self) -> i32 {
        return 26;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 26 {
            return Some(MangrovePlanks {
            });
        }
        return None;
    }
}

