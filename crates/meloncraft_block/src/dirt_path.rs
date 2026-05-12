use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirtPath {
}


impl BlockState for DirtPath {
    fn to_id(&self) -> i32 {
        return 14613;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14613 {
            return Some(DirtPath {
            });
        }
        return None;
    }
}

