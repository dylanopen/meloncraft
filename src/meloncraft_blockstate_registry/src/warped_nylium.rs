use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WarpedNylium {}

impl BlockState for WarpedNylium {
    fn to_id(&self) -> i32 {
        return 20755;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20755 {
            return Some(WarpedNylium {});
        }
        return None;
    }
}
