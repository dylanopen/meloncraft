use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OakPlanks {}

impl BlockState for OakPlanks {
    fn to_id(&self) -> i32 {
        return 15;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 15 {
            return Some(OakPlanks {});
        }
        return None;
    }
}
