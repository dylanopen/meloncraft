use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KelpPlant {}

impl BlockState for KelpPlant {
    fn to_id(&self) -> i32 {
        return 14886;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14886 {
            return Some(KelpPlant {});
        }
        return None;
    }
}
