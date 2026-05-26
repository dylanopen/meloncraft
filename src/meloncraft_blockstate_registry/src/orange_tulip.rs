use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrangeTulip {}

impl BlockState for OrangeTulip {
    fn to_id(&self) -> i32 {
        return 2128;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2128 {
            return Some(OrangeTulip {});
        }
        return None;
    }
}
