use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GildedBlackstone {
}


impl BlockState for GildedBlackstone {
    fn to_id(&self) -> i32 {
        return 22454;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22454 {
            return Some(GildedBlackstone {
            });
        }
        return None;
    }
}

