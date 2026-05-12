use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrayCarpet {
}


impl BlockState for GrayCarpet {
    fn to_id(&self) -> i32 {
        return 12701;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12701 {
            return Some(GrayCarpet {
            });
        }
        return None;
    }
}

