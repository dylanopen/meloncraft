use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedRedTulip {}

impl BlockState for PottedRedTulip {
    fn to_id(&self) -> i32 {
        return 10445;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10445 {
            return Some(PottedRedTulip {});
        }
        return None;
    }
}
