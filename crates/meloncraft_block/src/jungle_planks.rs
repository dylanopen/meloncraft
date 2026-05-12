use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JunglePlanks {
}


impl BlockState for JunglePlanks {
    fn to_id(self) -> i32 {
        return 18;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 18 {
            return Some(JunglePlanks {
            });
        }
        return None;
    }
}

