use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LapisOre {
}


impl BlockState for LapisOre {
    fn to_id(self) -> i32 {
        return 563;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 563 {
            return Some(LapisOre {
            });
        }
        return None;
    }
}

