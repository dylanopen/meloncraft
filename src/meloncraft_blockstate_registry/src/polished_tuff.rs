use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedTuff {}

impl BlockState for PolishedTuff {
    fn to_id(&self) -> i32 {
        return 23661;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 23661 {
            return Some(PolishedTuff {});
        }
        return None;
    }
}
