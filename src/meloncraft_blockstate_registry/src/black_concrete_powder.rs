use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackConcretePowder {}

impl BlockState for BlackConcretePowder {
    fn to_id(&self) -> i32 {
        return 14859;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14859 {
            return Some(BlackConcretePowder {});
        }
        return None;
    }
}
