use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinkStainedGlass {
}


impl BlockState for PinkStainedGlass {
    fn to_id(&self) -> i32 {
        return 6903;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6903 {
            return Some(PinkStainedGlass {
            });
        }
        return None;
    }
}

