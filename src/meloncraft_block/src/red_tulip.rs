use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedTulip {
}


impl BlockState for RedTulip {
    fn to_id(&self) -> i32 {
        return 2127;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2127 {
            return Some(RedTulip {
            });
        }
        return None;
    }
}

