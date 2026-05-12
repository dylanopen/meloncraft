use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Seagrass {
}


impl BlockState for Seagrass {
    fn to_id(&self) -> i32 {
        return 2054;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2054 {
            return Some(Seagrass {
            });
        }
        return None;
    }
}

