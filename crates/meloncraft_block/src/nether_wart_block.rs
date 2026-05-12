use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetherWartBlock {
}


impl BlockState for NetherWartBlock {
    fn to_id(&self) -> i32 {
        return 14644;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14644 {
            return Some(NetherWartBlock {
            });
        }
        return None;
    }
}

