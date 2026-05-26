use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreenCarpet {}

impl BlockState for GreenCarpet {
    fn to_id(&self) -> i32 {
        return 12707;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12707 {
            return Some(GreenCarpet {});
        }
        return None;
    }
}
