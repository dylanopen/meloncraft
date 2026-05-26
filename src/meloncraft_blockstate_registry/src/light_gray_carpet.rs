use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightGrayCarpet {}

impl BlockState for LightGrayCarpet {
    fn to_id(&self) -> i32 {
        return 12702;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12702 {
            return Some(LightGrayCarpet {});
        }
        return None;
    }
}
