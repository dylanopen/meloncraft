use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedMushroom {}

impl BlockState for RedMushroom {
    fn to_id(&self) -> i32 {
        return 2136;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2136 {
            return Some(RedMushroom {});
        }
        return None;
    }
}
