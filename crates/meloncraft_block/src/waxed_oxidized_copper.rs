use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedOxidizedCopper {
}


impl BlockState for WaxedOxidizedCopper {
    fn to_id(self) -> i32 {
        return 25472;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25472 {
            return Some(WaxedOxidizedCopper {
            });
        }
        return None;
    }
}

