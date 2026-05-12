use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrownMushroom {
}


impl BlockState for BrownMushroom {
    fn to_id(self) -> i32 {
        return 2135;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2135 {
            return Some(BrownMushroom {
            });
        }
        return None;
    }
}

