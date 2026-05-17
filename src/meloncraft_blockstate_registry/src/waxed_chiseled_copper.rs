use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedChiseledCopper {
}


impl BlockState for WaxedChiseledCopper {
    fn to_id(&self) -> i32 {
        return 25124;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25124 {
            return Some(WaxedChiseledCopper {
            });
        }
        return None;
    }
}

