use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndPortal {
}


impl BlockState for EndPortal {
    fn to_id(&self) -> i32 {
        return 9267;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9267 {
            return Some(EndPortal {
            });
        }
        return None;
    }
}

