use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoulSand {}

impl BlockState for SoulSand {
    fn to_id(&self) -> i32 {
        return 6797;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6797 {
            return Some(SoulSand {});
        }
        return None;
    }
}
