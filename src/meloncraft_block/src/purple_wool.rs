use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurpleWool {
}


impl BlockState for PurpleWool {
    fn to_id(&self) -> i32 {
        return 2103;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2103 {
            return Some(PurpleWool {
            });
        }
        return None;
    }
}

