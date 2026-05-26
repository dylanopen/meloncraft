use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChiseledRedSandstone {}

impl BlockState for ChiseledRedSandstone {
    fn to_id(&self) -> i32 {
        return 13046;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13046 {
            return Some(ChiseledRedSandstone {});
        }
        return None;
    }
}
