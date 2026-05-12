use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuartzBricks {
}


impl BlockState for QuartzBricks {
    fn to_id(self) -> i32 {
        return 22893;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22893 {
            return Some(QuartzBricks {
            });
        }
        return None;
    }
}

