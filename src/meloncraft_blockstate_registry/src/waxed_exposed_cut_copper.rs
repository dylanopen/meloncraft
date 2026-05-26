use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedExposedCutCopper {}

impl BlockState for WaxedExposedCutCopper {
    fn to_id(&self) -> i32 {
        return 25475;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25475 {
            return Some(WaxedExposedCutCopper {});
        }
        return None;
    }
}
