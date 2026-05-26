use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Andesite {}

impl BlockState for Andesite {
    fn to_id(&self) -> i32 {
        return 6;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6 {
            return Some(Andesite {});
        }
        return None;
    }
}
