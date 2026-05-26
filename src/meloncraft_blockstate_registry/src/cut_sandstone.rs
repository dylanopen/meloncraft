use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CutSandstone {}

impl BlockState for CutSandstone {
    fn to_id(&self) -> i32 {
        return 580;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 580 {
            return Some(CutSandstone {});
        }
        return None;
    }
}
