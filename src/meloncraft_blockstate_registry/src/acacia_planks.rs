use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcaciaPlanks {
}


impl BlockState for AcaciaPlanks {
    fn to_id(&self) -> i32 {
        return 19;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 19 {
            return Some(AcaciaPlanks {
            });
        }
        return None;
    }
}

