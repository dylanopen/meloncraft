use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedPaleOakSapling {}

impl BlockState for PottedPaleOakSapling {
    fn to_id(&self) -> i32 {
        return 10437;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10437 {
            return Some(PottedPaleOakSapling {});
        }
        return None;
    }
}
