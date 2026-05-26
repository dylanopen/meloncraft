use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmethystBlock {}

impl BlockState for AmethystBlock {
    fn to_id(&self) -> i32 {
        return 23200;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23200 {
            return Some(AmethystBlock {});
        }
        return None;
    }
}
