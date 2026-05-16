use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LimeTerracotta {
}


impl BlockState for LimeTerracotta {
    fn to_id(&self) -> i32 {
        return 11247;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11247 {
            return Some(LimeTerracotta {
            });
        }
        return None;
    }
}

