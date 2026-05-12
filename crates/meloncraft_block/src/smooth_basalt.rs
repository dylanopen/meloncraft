use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmoothBasalt {
}


impl BlockState for SmoothBasalt {
    fn to_id(self) -> i32 {
        return 29374;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29374 {
            return Some(SmoothBasalt {
            });
        }
        return None;
    }
}

