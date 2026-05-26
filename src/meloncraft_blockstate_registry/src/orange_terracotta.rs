use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrangeTerracotta {}

impl BlockState for OrangeTerracotta {
    fn to_id(&self) -> i32 {
        return 11243;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11243 {
            return Some(OrangeTerracotta {});
        }
        return None;
    }
}
