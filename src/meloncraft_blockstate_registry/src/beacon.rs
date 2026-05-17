use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Beacon {
}


impl BlockState for Beacon {
    fn to_id(&self) -> i32 {
        return 9779;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9779 {
            return Some(Beacon {
            });
        }
        return None;
    }
}

