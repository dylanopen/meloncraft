use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlueTerracotta {
}


impl BlockState for BlueTerracotta {
    fn to_id(self) -> i32 {
        return 11253;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11253 {
            return Some(BlueTerracotta {
            });
        }
        return None;
    }
}

