use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeatheredChiseledCopper {}

impl BlockState for WeatheredChiseledCopper {
    fn to_id(&self) -> i32 {
        return 25118;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25118 {
            return Some(WeatheredChiseledCopper {});
        }
        return None;
    }
}
