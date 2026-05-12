use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredCopper {
}


impl BlockState for WaxedWeatheredCopper {
    fn to_id(&self) -> i32 {
        return 25470;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25470 {
            return Some(WaxedWeatheredCopper {
            });
        }
        return None;
    }
}

