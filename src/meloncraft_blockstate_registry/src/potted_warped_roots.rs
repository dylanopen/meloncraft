use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedWarpedRoots {}

impl BlockState for PottedWarpedRoots {
    fn to_id(&self) -> i32 {
        return 21627;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21627 {
            return Some(PottedWarpedRoots {});
        }
        return None;
    }
}
