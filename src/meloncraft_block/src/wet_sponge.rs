use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WetSponge {
}


impl BlockState for WetSponge {
    fn to_id(&self) -> i32 {
        return 561;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 561 {
            return Some(WetSponge {
            });
        }
        return None;
    }
}

