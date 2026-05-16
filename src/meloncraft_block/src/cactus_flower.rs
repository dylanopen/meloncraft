use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CactusFlower {
}


impl BlockState for CactusFlower {
    fn to_id(&self) -> i32 {
        return 6744;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6744 {
            return Some(CactusFlower {
            });
        }
        return None;
    }
}

