use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BrownCarpet {}

impl BlockState for BrownCarpet {
    fn to_id(&self) -> i32 {
        return 12706;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12706 {
            return Some(BrownCarpet {});
        }
        return None;
    }
}
