use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedWhiteTulip {
}


impl BlockState for PottedWhiteTulip {
    fn to_id(&self) -> i32 {
        return 10447;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10447 {
            return Some(PottedWhiteTulip {
            });
        }
        return None;
    }
}

