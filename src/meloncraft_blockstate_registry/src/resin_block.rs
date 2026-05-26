use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResinBlock {}

impl BlockState for ResinBlock {
    fn to_id(&self) -> i32 {
        return 8720;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8720 {
            return Some(ResinBlock {});
        }
        return None;
    }
}
