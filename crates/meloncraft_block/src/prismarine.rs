use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Prismarine {
}


impl BlockState for Prismarine {
    fn to_id(self) -> i32 {
        return 12429;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12429 {
            return Some(Prismarine {
            });
        }
        return None;
    }
}

