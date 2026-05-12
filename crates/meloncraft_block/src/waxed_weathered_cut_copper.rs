use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WaxedWeatheredCutCopper {
}


impl BlockState for WaxedWeatheredCutCopper {
    fn to_id(self) -> i32 {
        return 25474;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 25474 {
            return Some(WaxedWeatheredCutCopper {
            });
        }
        return None;
    }
}

