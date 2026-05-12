use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sand {
}


impl BlockState for Sand {
    fn to_id(self) -> i32 {
        return 118;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 118 {
            return Some(Sand {
            });
        }
        return None;
    }
}

