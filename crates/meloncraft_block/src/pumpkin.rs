use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pumpkin {
}


impl BlockState for Pumpkin {
    fn to_id(&self) -> i32 {
        return 8131;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 8131 {
            return Some(Pumpkin {
            });
        }
        return None;
    }
}

