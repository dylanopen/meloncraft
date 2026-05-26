use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Netherrack {}

impl BlockState for Netherrack {
    fn to_id(&self) -> i32 {
        return 6796;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6796 {
            return Some(Netherrack {});
        }
        return None;
    }
}
