use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TintedGlass {}

impl BlockState for TintedGlass {
    fn to_id(&self) -> i32 {
        return 24486;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 24486 {
            return Some(TintedGlass {});
        }
        return None;
    }
}
