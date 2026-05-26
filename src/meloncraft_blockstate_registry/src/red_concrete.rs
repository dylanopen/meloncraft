use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedConcrete {}

impl BlockState for RedConcrete {
    fn to_id(&self) -> i32 {
        return 14842;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14842 {
            return Some(RedConcrete {});
        }
        return None;
    }
}
