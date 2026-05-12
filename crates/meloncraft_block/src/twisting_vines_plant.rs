use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TwistingVinesPlant {
}


impl BlockState for TwistingVinesPlant {
    fn to_id(self) -> i32 {
        return 20828;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20828 {
            return Some(TwistingVinesPlant {
            });
        }
        return None;
    }
}

