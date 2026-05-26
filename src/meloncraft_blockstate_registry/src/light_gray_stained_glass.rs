use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LightGrayStainedGlass {}

impl BlockState for LightGrayStainedGlass {
    fn to_id(&self) -> i32 {
        return 6905;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6905 {
            return Some(LightGrayStainedGlass {});
        }
        return None;
    }
}
