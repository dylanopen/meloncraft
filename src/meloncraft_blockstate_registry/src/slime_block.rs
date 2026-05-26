use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlimeBlock {}

impl BlockState for SlimeBlock {
    fn to_id(&self) -> i32 {
        return 12330;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12330 {
            return Some(SlimeBlock {});
        }
        return None;
    }
}
