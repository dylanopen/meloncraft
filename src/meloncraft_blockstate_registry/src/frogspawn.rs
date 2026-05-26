use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Frogspawn {}

impl BlockState for Frogspawn {
    fn to_id(&self) -> i32 {
        return 29389;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 29389 {
            return Some(Frogspawn {});
        }
        return None;
    }
}
