use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WitherRose {
}


impl BlockState for WitherRose {
    fn to_id(&self) -> i32 {
        return 2133;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2133 {
            return Some(WitherRose {
            });
        }
        return None;
    }
}

