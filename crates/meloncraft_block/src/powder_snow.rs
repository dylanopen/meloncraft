use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PowderSnow {
}


impl BlockState for PowderSnow {
    fn to_id(&self) -> i32 {
        return 24487;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 24487 {
            return Some(PowderSnow {
            });
        }
        return None;
    }
}

