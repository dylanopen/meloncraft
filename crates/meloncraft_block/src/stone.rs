use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stone {
}


impl BlockState for Stone {
    fn to_id(&self) -> i32 {
        return 1;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 1 {
            return Some(Stone {
            });
        }
        return None;
    }
}

