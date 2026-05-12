use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fern {
}


impl BlockState for Fern {
    fn to_id(self) -> i32 {
        return 2049;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2049 {
            return Some(Fern {
            });
        }
        return None;
    }
}

