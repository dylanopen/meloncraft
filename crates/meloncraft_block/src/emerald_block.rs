use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmeraldBlock {
}


impl BlockState for EmeraldBlock {
    fn to_id(self) -> i32 {
        return 9526;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9526 {
            return Some(EmeraldBlock {
            });
        }
        return None;
    }
}

