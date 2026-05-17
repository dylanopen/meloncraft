use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CyanTerracotta {
}


impl BlockState for CyanTerracotta {
    fn to_id(&self) -> i32 {
        return 11251;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11251 {
            return Some(CyanTerracotta {
            });
        }
        return None;
    }
}

