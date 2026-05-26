use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrayWool {}

impl BlockState for GrayWool {
    fn to_id(&self) -> i32 {
        return 2100;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2100 {
            return Some(GrayWool {});
        }
        return None;
    }
}
