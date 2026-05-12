use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedBlackstone {
}


impl BlockState for PolishedBlackstone {
    fn to_id(&self) -> i32 {
        return 22040;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22040 {
            return Some(PolishedBlackstone {
            });
        }
        return None;
    }
}

