use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurpurBlock {}

impl BlockState for PurpurBlock {
    fn to_id(&self) -> i32 {
        return 14510;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14510 {
            return Some(PurpurBlock {});
        }
        return None;
    }
}
