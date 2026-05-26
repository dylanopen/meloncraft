use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PinkTerracotta {}

impl BlockState for PinkTerracotta {
    fn to_id(&self) -> i32 {
        return 11248;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11248 {
            return Some(PinkTerracotta {});
        }
        return None;
    }
}
