use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedMangrovePropagule {
}


impl BlockState for PottedMangrovePropagule {
    fn to_id(self) -> i32 {
        return 10438;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10438 {
            return Some(PottedMangrovePropagule {
            });
        }
        return None;
    }
}

