use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CherryPlanks {
}


impl BlockState for CherryPlanks {
    fn to_id(self) -> i32 {
        return 20;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20 {
            return Some(CherryPlanks {
            });
        }
        return None;
    }
}

