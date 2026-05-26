use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhiteConcretePowder {}

impl BlockState for WhiteConcretePowder {
    fn to_id(&self) -> i32 {
        return 14844;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14844 {
            return Some(WhiteConcretePowder {});
        }
        return None;
    }
}
