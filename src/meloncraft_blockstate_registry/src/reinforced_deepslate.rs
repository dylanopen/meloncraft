use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReinforcedDeepslate {}

impl BlockState for ReinforcedDeepslate {
    fn to_id(&self) -> i32 {
        return 29390;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29390 {
            return Some(ReinforcedDeepslate {});
        }
        return None;
    }
}
