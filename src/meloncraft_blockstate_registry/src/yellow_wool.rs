use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YellowWool {}

impl BlockState for YellowWool {
    fn to_id(&self) -> i32 {
        return 2097;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2097 {
            return Some(YellowWool {});
        }
        return None;
    }
}
