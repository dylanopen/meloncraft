use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChiseledCopper {}

impl BlockState for ChiseledCopper {
    fn to_id(&self) -> i32 {
        return 25120;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25120 {
            return Some(ChiseledCopper {});
        }
        return None;
    }
}
