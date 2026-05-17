use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoulFire {
}


impl BlockState for SoulFire {
    fn to_id(&self) -> i32 {
        return 3686;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 3686 {
            return Some(SoulFire {
            });
        }
        return None;
    }
}

