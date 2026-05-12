use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrackedNetherBricks {
}


impl BlockState for CrackedNetherBricks {
    fn to_id(&self) -> i32 {
        return 22892;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22892 {
            return Some(CrackedNetherBricks {
            });
        }
        return None;
    }
}

