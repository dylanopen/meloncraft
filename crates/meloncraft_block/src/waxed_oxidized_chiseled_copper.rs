use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedOxidizedChiseledCopper {
}


impl BlockState for WaxedOxidizedChiseledCopper {
    fn to_id(self) -> i32 {
        return 25121;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25121 {
            return Some(WaxedOxidizedChiseledCopper {
            });
        }
        return None;
    }
}

