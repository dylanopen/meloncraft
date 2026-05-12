use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeepslateCopperOre {
}


impl BlockState for DeepslateCopperOre {
    fn to_id(&self) -> i32 {
        return 25112;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25112 {
            return Some(DeepslateCopperOre {
            });
        }
        return None;
    }
}

