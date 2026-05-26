use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadFireCoralBlock {}

impl BlockState for DeadFireCoralBlock {
    fn to_id(&self) -> i32 {
        return 14938;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14938 {
            return Some(DeadFireCoralBlock {});
        }
        return None;
    }
}
