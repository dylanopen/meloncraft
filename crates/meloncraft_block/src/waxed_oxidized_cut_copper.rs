use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedOxidizedCutCopper {
}


impl BlockState for WaxedOxidizedCutCopper {
    fn to_id(self) -> i32 {
        return 25473;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25473 {
            return Some(WaxedOxidizedCutCopper {
            });
        }
        return None;
    }
}

