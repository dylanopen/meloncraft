use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Blackstone {}

impl BlockState for Blackstone {
    fn to_id(&self) -> i32 {
        return 21629;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 21629 {
            return Some(Blackstone {});
        }
        return None;
    }
}
