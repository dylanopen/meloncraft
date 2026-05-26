use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnchantingTable {}

impl BlockState for EnchantingTable {
    fn to_id(&self) -> i32 {
        return 9250;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9250 {
            return Some(EnchantingTable {});
        }
        return None;
    }
}
