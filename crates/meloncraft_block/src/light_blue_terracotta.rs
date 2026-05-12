use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightBlueTerracotta {
}


impl BlockState for LightBlueTerracotta {
    fn to_id(self) -> i32 {
        return 11245;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11245 {
            return Some(LightBlueTerracotta {
            });
        }
        return None;
    }
}

