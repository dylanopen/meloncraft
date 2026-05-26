use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreenConcrete {}

impl BlockState for GreenConcrete {
    fn to_id(&self) -> i32 {
        return 14841;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14841 {
            return Some(GreenConcrete {});
        }
        return None;
    }
}
