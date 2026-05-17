use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sculk {
}


impl BlockState for Sculk {
    fn to_id(&self) -> i32 {
        return 24968;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 24968 {
            return Some(Sculk {
            });
        }
        return None;
    }
}

