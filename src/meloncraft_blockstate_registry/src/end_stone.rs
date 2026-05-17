use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndStone {
}


impl BlockState for EndStone {
    fn to_id(&self) -> i32 {
        return 9276;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9276 {
            return Some(EndStone {
            });
        }
        return None;
    }
}

