use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CutCopper {
}


impl BlockState for CutCopper {
    fn to_id(&self) -> i32 {
        return 25116;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25116 {
            return Some(CutCopper {
            });
        }
        return None;
    }
}

