use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeadBush {
}


impl BlockState for DeadBush {
    fn to_id(&self) -> i32 {
        return 2050;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2050 {
            return Some(DeadBush {
            });
        }
        return None;
    }
}

