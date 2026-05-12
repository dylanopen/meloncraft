use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Torchflower {
}


impl BlockState for Torchflower {
    fn to_id(self) -> i32 {
        return 2122;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2122 {
            return Some(Torchflower {
            });
        }
        return None;
    }
}

