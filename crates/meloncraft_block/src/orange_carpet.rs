use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrangeCarpet {
}


impl BlockState for OrangeCarpet {
    fn to_id(self) -> i32 {
        return 12695;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12695 {
            return Some(OrangeCarpet {
            });
        }
        return None;
    }
}

