use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ice {
}


impl BlockState for Ice {
    fn to_id(self) -> i32 {
        return 6726;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6726 {
            return Some(Ice {
            });
        }
        return None;
    }
}

