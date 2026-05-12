use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightBlueStainedGlass {
}


impl BlockState for LightBlueStainedGlass {
    fn to_id(self) -> i32 {
        return 6900;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6900 {
            return Some(LightBlueStainedGlass {
            });
        }
        return None;
    }
}

