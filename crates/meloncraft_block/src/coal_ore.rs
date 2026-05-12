use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoalOre {
}


impl BlockState for CoalOre {
    fn to_id(self) -> i32 {
        return 133;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 133 {
            return Some(CoalOre {
            });
        }
        return None;
    }
}

