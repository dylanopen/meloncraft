use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResinBricks {
}


impl BlockState for ResinBricks {
    fn to_id(self) -> i32 {
        return 8721;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8721 {
            return Some(ResinBricks {
            });
        }
        return None;
    }
}

