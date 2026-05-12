use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedWitherRose {
}


impl BlockState for PottedWitherRose {
    fn to_id(&self) -> i32 {
        return 10452;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10452 {
            return Some(PottedWitherRose {
            });
        }
        return None;
    }
}

