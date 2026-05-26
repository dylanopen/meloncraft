use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedOakSapling {}

impl BlockState for PottedOakSapling {
    fn to_id(&self) -> i32 {
        return 10430;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10430 {
            return Some(PottedOakSapling {});
        }
        return None;
    }
}
