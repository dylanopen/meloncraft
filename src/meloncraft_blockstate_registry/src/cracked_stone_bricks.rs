use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrackedStoneBricks {}

impl BlockState for CrackedStoneBricks {
    fn to_id(&self) -> i32 {
        return 7555;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 7555 {
            return Some(CrackedStoneBricks {});
        }
        return None;
    }
}
