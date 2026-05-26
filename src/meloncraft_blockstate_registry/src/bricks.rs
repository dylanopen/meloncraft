use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bricks {}

impl BlockState for Bricks {
    fn to_id(&self) -> i32 {
        return 2139;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2139 {
            return Some(Bricks {});
        }
        return None;
    }
}
