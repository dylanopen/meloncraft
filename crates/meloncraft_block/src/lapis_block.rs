use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LapisBlock {
}


impl BlockState for LapisBlock {
    fn to_id(&self) -> i32 {
        return 565;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 565 {
            return Some(LapisBlock {
            });
        }
        return None;
    }
}

