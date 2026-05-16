use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhiteTerracotta {
}


impl BlockState for WhiteTerracotta {
    fn to_id(&self) -> i32 {
        return 11242;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11242 {
            return Some(WhiteTerracotta {
            });
        }
        return None;
    }
}

