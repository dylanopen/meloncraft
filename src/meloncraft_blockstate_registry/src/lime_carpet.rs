use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LimeCarpet {}

impl BlockState for LimeCarpet {
    fn to_id(&self) -> i32 {
        return 12699;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 12699 {
            return Some(LimeCarpet {});
        }
        return None;
    }
}
