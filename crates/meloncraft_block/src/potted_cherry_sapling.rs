use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedCherrySapling {
}


impl BlockState for PottedCherrySapling {
    fn to_id(self) -> i32 {
        return 10435;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10435 {
            return Some(PottedCherrySapling {
            });
        }
        return None;
    }
}

