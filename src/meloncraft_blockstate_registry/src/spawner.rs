use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Spawner {}

impl BlockState for Spawner {
    fn to_id(&self) -> i32 {
        return 3687;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 3687 {
            return Some(Spawner {});
        }
        return None;
    }
}
