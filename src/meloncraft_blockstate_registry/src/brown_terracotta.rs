use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrownTerracotta {
}


impl BlockState for BrownTerracotta {
    fn to_id(&self) -> i32 {
        return 11254;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11254 {
            return Some(BrownTerracotta {
            });
        }
        return None;
    }
}

