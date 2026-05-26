use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoneBricks {}

impl BlockState for StoneBricks {
    fn to_id(&self) -> i32 {
        return 7553;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7553 {
            return Some(StoneBricks {});
        }
        return None;
    }
}
