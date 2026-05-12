use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuartzBlock {
}


impl BlockState for QuartzBlock {
    fn to_id(self) -> i32 {
        return 11121;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11121 {
            return Some(QuartzBlock {
            });
        }
        return None;
    }
}

