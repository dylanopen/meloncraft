use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeepslateBricks {
}


impl BlockState for DeepslateBricks {
    fn to_id(&self) -> i32 {
        return 28957;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 28957 {
            return Some(DeepslateBricks {
            });
        }
        return None;
    }
}

