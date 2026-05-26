use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LavaCauldron {}

impl BlockState for LavaCauldron {
    fn to_id(&self) -> i32 {
        return 9263;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9263 {
            return Some(LavaCauldron {});
        }
        return None;
    }
}
