use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredChiseledCopper {}

impl BlockState for WaxedWeatheredChiseledCopper {
    fn to_id(&self) -> i32 {
        return 25122;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25122 {
            return Some(WaxedWeatheredChiseledCopper {});
        }
        return None;
    }
}
