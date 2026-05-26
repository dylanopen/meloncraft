use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetherQuartzOre {}

impl BlockState for NetherQuartzOre {
    fn to_id(&self) -> i32 {
        return 11110;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 11110 {
            return Some(NetherQuartzOre {});
        }
        return None;
    }
}
