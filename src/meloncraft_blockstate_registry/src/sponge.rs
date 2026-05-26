use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sponge {}

impl BlockState for Sponge {
    fn to_id(&self) -> i32 {
        return 560;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 560 {
            return Some(Sponge {});
        }
        return None;
    }
}
