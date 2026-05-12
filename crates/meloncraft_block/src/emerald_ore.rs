use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmeraldOre {
}


impl BlockState for EmeraldOre {
    fn to_id(&self) -> i32 {
        return 9372;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9372 {
            return Some(EmeraldOre {
            });
        }
        return None;
    }
}

