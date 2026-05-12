use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IronBlock {
}


impl BlockState for IronBlock {
    fn to_id(self) -> i32 {
        return 2138;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2138 {
            return Some(IronBlock {
            });
        }
        return None;
    }
}

