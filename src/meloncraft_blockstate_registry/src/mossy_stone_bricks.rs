use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MossyStoneBricks {}

impl BlockState for MossyStoneBricks {
    fn to_id(&self) -> i32 {
        return 7554;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7554 {
            return Some(MossyStoneBricks {});
        }
        return None;
    }
}
