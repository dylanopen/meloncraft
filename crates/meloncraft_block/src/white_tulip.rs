use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhiteTulip {
}


impl BlockState for WhiteTulip {
    fn to_id(self) -> i32 {
        return 2129;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2129 {
            return Some(WhiteTulip {
            });
        }
        return None;
    }
}

