use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortGrass {
}


impl BlockState for ShortGrass {
    fn to_id(&self) -> i32 {
        return 2048;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2048 {
            return Some(ShortGrass {
            });
        }
        return None;
    }
}

