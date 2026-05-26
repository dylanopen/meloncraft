use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmoothRedSandstone {}

impl BlockState for SmoothRedSandstone {
    fn to_id(&self) -> i32 {
        return 13281;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13281 {
            return Some(SmoothRedSandstone {});
        }
        return None;
    }
}
