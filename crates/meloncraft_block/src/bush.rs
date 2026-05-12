use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bush {
}


impl BlockState for Bush {
    fn to_id(self) -> i32 {
        return 2051;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2051 {
            return Some(Bush {
            });
        }
        return None;
    }
}

