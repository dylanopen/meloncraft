use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CopperBlock {
}


impl BlockState for CopperBlock {
    fn to_id(self) -> i32 {
        return 25107;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25107 {
            return Some(CopperBlock {
            });
        }
        return None;
    }
}

