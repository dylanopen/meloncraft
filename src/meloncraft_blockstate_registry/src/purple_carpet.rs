use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurpleCarpet {
}


impl BlockState for PurpleCarpet {
    fn to_id(&self) -> i32 {
        return 12704;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12704 {
            return Some(PurpleCarpet {
            });
        }
        return None;
    }
}

