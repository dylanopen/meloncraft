use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetheriteBlock {
}


impl BlockState for NetheriteBlock {
    fn to_id(&self) -> i32 {
        return 21616;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21616 {
            return Some(NetheriteBlock {
            });
        }
        return None;
    }
}

