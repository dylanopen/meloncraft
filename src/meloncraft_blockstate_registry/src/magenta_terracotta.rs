use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MagentaTerracotta {}

impl BlockState for MagentaTerracotta {
    fn to_id(&self) -> i32 {
        return 11244;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11244 {
            return Some(MagentaTerracotta {});
        }
        return None;
    }
}
