use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IronOre {
}


impl BlockState for IronOre {
    fn to_id(self) -> i32 {
        return 131;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 131 {
            return Some(IronOre {
            });
        }
        return None;
    }
}

