use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrayTerracotta {
}


impl BlockState for GrayTerracotta {
    fn to_id(self) -> i32 {
        return 11249;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11249 {
            return Some(GrayTerracotta {
            });
        }
        return None;
    }
}

