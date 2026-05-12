use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaleMossBlock {
}


impl BlockState for PaleMossBlock {
    fn to_id(self) -> i32 {
        return 29501;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29501 {
            return Some(PaleMossBlock {
            });
        }
        return None;
    }
}

