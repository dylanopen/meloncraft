use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhiteCarpet {}

impl BlockState for WhiteCarpet {
    fn to_id(&self) -> i32 {
        return 12694;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12694 {
            return Some(WhiteCarpet {});
        }
        return None;
    }
}
