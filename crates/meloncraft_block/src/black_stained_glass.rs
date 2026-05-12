use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackStainedGlass {
}


impl BlockState for BlackStainedGlass {
    fn to_id(self) -> i32 {
        return 6912;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6912 {
            return Some(BlackStainedGlass {
            });
        }
        return None;
    }
}

