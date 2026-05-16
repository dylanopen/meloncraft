use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlackTerracotta {
}


impl BlockState for BlackTerracotta {
    fn to_id(&self) -> i32 {
        return 11257;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11257 {
            return Some(BlackTerracotta {
            });
        }
        return None;
    }
}

