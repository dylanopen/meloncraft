use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HoneycombBlock {
}


impl BlockState for HoneycombBlock {
    fn to_id(self) -> i32 {
        return 21615;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21615 {
            return Some(HoneycombBlock {
            });
        }
        return None;
    }
}

