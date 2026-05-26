use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatheredCopper {}

impl BlockState for WeatheredCopper {
    fn to_id(&self) -> i32 {
        return 25109;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25109 {
            return Some(WeatheredCopper {});
        }
        return None;
    }
}
