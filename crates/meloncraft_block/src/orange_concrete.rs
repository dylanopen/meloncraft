use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrangeConcrete {
}


impl BlockState for OrangeConcrete {
    fn to_id(self) -> i32 {
        return 14829;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14829 {
            return Some(OrangeConcrete {
            });
        }
        return None;
    }
}

