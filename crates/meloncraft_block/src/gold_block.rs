use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GoldBlock {
}


impl BlockState for GoldBlock {
    fn to_id(self) -> i32 {
        return 2137;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2137 {
            return Some(GoldBlock {
            });
        }
        return None;
    }
}

