use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeepslateDiamondOre {
}


impl BlockState for DeepslateDiamondOre {
    fn to_id(&self) -> i32 {
        return 5107;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5107 {
            return Some(DeepslateDiamondOre {
            });
        }
        return None;
    }
}

