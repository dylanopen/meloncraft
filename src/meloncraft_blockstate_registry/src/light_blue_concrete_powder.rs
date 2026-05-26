use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightBlueConcretePowder {}

impl BlockState for LightBlueConcretePowder {
    fn to_id(&self) -> i32 {
        return 14847;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14847 {
            return Some(LightBlueConcretePowder {});
        }
        return None;
    }
}
