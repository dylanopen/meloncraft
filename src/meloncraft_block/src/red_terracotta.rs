use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedTerracotta {
}


impl BlockState for RedTerracotta {
    fn to_id(&self) -> i32 {
        return 11256;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11256 {
            return Some(RedTerracotta {
            });
        }
        return None;
    }
}

