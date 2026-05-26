use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExposedChiseledCopper {}

impl BlockState for ExposedChiseledCopper {
    fn to_id(&self) -> i32 {
        return 25119;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25119 {
            return Some(ExposedChiseledCopper {});
        }
        return None;
    }
}
