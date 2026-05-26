use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedClosedEyeblossom {}

impl BlockState for PottedClosedEyeblossom {
    fn to_id(&self) -> i32 {
        return 29669;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29669 {
            return Some(PottedClosedEyeblossom {});
        }
        return None;
    }
}
