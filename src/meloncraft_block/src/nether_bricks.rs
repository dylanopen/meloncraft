use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetherBricks {
}


impl BlockState for NetherBricks {
    fn to_id(&self) -> i32 {
        return 9133;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9133 {
            return Some(NetherBricks {
            });
        }
        return None;
    }
}

