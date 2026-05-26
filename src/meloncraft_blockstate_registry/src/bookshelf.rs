use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bookshelf {}

impl BlockState for Bookshelf {
    fn to_id(&self) -> i32 {
        return 2142;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2142 {
            return Some(Bookshelf {});
        }
        return None;
    }
}
