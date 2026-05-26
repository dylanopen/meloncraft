use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedBlackstoneBricks {}

impl BlockState for PolishedBlackstoneBricks {
    fn to_id(&self) -> i32 {
        return 22041;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 22041 {
            return Some(PolishedBlackstoneBricks {});
        }
        return None;
    }
}
