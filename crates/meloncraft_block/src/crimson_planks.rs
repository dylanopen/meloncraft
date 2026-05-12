use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrimsonPlanks {
}


impl BlockState for CrimsonPlanks {
    fn to_id(self) -> i32 {
        return 20830;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 20830 {
            return Some(CrimsonPlanks {
            });
        }
        return None;
    }
}

