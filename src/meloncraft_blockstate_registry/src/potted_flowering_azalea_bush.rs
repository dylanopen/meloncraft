use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedFloweringAzaleaBush {}

impl BlockState for PottedFloweringAzaleaBush {
    fn to_id(&self) -> i32 {
        return 29379;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29379 {
            return Some(PottedFloweringAzaleaBush {});
        }
        return None;
    }
}
