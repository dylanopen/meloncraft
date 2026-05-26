use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrangeWool {}

impl BlockState for OrangeWool {
    fn to_id(&self) -> i32 {
        return 2094;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2094 {
            return Some(OrangeWool {});
        }
        return None;
    }
}
