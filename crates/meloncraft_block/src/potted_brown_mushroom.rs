use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedBrownMushroom {
}


impl BlockState for PottedBrownMushroom {
    fn to_id(self) -> i32 {
        return 10454;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10454 {
            return Some(PottedBrownMushroom {
            });
        }
        return None;
    }
}

