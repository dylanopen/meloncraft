use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LilyPad {
}


impl BlockState for LilyPad {
    fn to_id(self) -> i32 {
        return 8719;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8719 {
            return Some(LilyPad {
            });
        }
        return None;
    }
}

