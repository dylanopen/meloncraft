use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinkTulip {
}


impl BlockState for PinkTulip {
    fn to_id(self) -> i32 {
        return 2130;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2130 {
            return Some(PinkTulip {
            });
        }
        return None;
    }
}

