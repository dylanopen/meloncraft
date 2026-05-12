use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BambooPlanks {
}


impl BlockState for BambooPlanks {
    fn to_id(self) -> i32 {
        return 27;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27 {
            return Some(BambooPlanks {
            });
        }
        return None;
    }
}

