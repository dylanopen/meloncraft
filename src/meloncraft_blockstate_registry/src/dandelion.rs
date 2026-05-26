use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dandelion {}

impl BlockState for Dandelion {
    fn to_id(&self) -> i32 {
        return 2121;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2121 {
            return Some(Dandelion {});
        }
        return None;
    }
}
