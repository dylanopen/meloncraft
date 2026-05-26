use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagmaBlock {}

impl BlockState for MagmaBlock {
    fn to_id(&self) -> i32 {
        return 14643;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14643 {
            return Some(MagmaBlock {});
        }
        return None;
    }
}
