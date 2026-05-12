use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BirchPlanks {
}


impl BlockState for BirchPlanks {
    fn to_id(&self) -> i32 {
        return 17;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 17 {
            return Some(BirchPlanks {
            });
        }
        return None;
    }
}

