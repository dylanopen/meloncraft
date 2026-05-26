use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Poppy {}

impl BlockState for Poppy {
    fn to_id(&self) -> i32 {
        return 2123;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2123 {
            return Some(Poppy {});
        }
        return None;
    }
}
