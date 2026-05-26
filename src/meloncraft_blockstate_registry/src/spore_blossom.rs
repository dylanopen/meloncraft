use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SporeBlossom {}

impl BlockState for SporeBlossom {
    fn to_id(&self) -> i32 {
        return 27608;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 27608 {
            return Some(SporeBlossom {});
        }
        return None;
    }
}
