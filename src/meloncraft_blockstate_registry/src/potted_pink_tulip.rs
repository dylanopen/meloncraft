use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedPinkTulip {}

impl BlockState for PottedPinkTulip {
    fn to_id(&self) -> i32 {
        return 10448;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10448 {
            return Some(PottedPinkTulip {});
        }
        return None;
    }
}
