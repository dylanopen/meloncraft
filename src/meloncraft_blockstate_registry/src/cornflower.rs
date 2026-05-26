use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cornflower {}

impl BlockState for Cornflower {
    fn to_id(&self) -> i32 {
        return 2132;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2132 {
            return Some(Cornflower {});
        }
        return None;
    }
}
