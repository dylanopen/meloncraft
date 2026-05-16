use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiamondOre {
}


impl BlockState for DiamondOre {
    fn to_id(&self) -> i32 {
        return 5106;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5106 {
            return Some(DiamondOre {
            });
        }
        return None;
    }
}

