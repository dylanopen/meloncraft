use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlueWool {}

impl BlockState for BlueWool {
    fn to_id(&self) -> i32 {
        return 2104;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2104 {
            return Some(BlueWool {});
        }
        return None;
    }
}
