use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gravel {
}


impl BlockState for Gravel {
    fn to_id(self) -> i32 {
        return 124;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 124 {
            return Some(Gravel {
            });
        }
        return None;
    }
}

