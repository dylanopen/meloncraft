use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiamondBlock {}

impl BlockState for DiamondBlock {
    fn to_id(&self) -> i32 {
        return 5108;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5108 {
            return Some(DiamondBlock {});
        }
        return None;
    }
}
