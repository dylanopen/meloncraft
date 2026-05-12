use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sandstone {
}


impl BlockState for Sandstone {
    fn to_id(self) -> i32 {
        return 578;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 578 {
            return Some(Sandstone {
            });
        }
        return None;
    }
}

