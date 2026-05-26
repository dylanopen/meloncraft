use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cauldron {}

impl BlockState for Cauldron {
    fn to_id(&self) -> i32 {
        return 9259;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9259 {
            return Some(Cauldron {});
        }
        return None;
    }
}
