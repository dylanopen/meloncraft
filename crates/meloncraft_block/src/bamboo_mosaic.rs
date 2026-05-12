use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BambooMosaic {
}


impl BlockState for BambooMosaic {
    fn to_id(&self) -> i32 {
        return 28;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 28 {
            return Some(BambooMosaic {
            });
        }
        return None;
    }
}

