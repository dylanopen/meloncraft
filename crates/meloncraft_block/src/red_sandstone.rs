use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedSandstone {
}


impl BlockState for RedSandstone {
    fn to_id(&self) -> i32 {
        return 13045;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 13045 {
            return Some(RedSandstone {
            });
        }
        return None;
    }
}

