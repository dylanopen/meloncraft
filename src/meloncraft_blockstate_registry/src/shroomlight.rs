use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shroomlight {}

impl BlockState for Shroomlight {
    fn to_id(&self) -> i32 {
        return 20774;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20774 {
            return Some(Shroomlight {});
        }
        return None;
    }
}
