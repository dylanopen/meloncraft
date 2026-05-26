use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cobweb {}

impl BlockState for Cobweb {
    fn to_id(&self) -> i32 {
        return 2047;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2047 {
            return Some(Cobweb {});
        }
        return None;
    }
}
