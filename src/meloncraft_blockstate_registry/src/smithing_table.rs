use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmithingTable {}

impl BlockState for SmithingTable {
    fn to_id(&self) -> i32 {
        return 20598;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20598 {
            return Some(SmithingTable {});
        }
        return None;
    }
}
