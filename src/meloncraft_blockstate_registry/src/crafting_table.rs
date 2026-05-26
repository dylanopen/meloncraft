use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CraftingTable {}

impl BlockState for CraftingTable {
    fn to_id(&self) -> i32 {
        return 5109;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5109 {
            return Some(CraftingTable {});
        }
        return None;
    }
}
