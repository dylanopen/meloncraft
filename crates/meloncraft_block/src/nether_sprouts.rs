use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetherSprouts {
}


impl BlockState for NetherSprouts {
    fn to_id(self) -> i32 {
        return 20759;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20759 {
            return Some(NetherSprouts {
            });
        }
        return None;
    }
}

