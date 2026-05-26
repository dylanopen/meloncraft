use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CyanStainedGlass {}

impl BlockState for CyanStainedGlass {
    fn to_id(&self) -> i32 {
        return 6906;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6906 {
            return Some(CyanStainedGlass {});
        }
        return None;
    }
}
