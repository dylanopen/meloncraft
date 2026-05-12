use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MossBlock {
}


impl BlockState for MossBlock {
    fn to_id(self) -> i32 {
        return 27660;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27660 {
            return Some(MossBlock {
            });
        }
        return None;
    }
}

