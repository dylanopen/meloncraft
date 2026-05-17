use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurpleTerracotta {
}


impl BlockState for PurpleTerracotta {
    fn to_id(&self) -> i32 {
        return 11252;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11252 {
            return Some(PurpleTerracotta {
            });
        }
        return None;
    }
}

