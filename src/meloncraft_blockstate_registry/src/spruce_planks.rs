use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SprucePlanks {}

impl BlockState for SprucePlanks {
    fn to_id(&self) -> i32 {
        return 16;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 16 {
            return Some(SprucePlanks {});
        }
        return None;
    }
}
