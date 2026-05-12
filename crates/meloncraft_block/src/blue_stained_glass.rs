use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlueStainedGlass {
}


impl BlockState for BlueStainedGlass {
    fn to_id(self) -> i32 {
        return 6908;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6908 {
            return Some(BlueStainedGlass {
            });
        }
        return None;
    }
}

