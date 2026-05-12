use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedRedMushroom {
}


impl BlockState for PottedRedMushroom {
    fn to_id(&self) -> i32 {
        return 10453;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10453 {
            return Some(PottedRedMushroom {
            });
        }
        return None;
    }
}

