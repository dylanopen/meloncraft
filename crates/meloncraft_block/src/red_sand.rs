use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedSand {
}


impl BlockState for RedSand {
    fn to_id(&self) -> i32 {
        return 123;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 123 {
            return Some(RedSand {
            });
        }
        return None;
    }
}

