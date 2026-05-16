use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedBirchSapling {
}


impl BlockState for PottedBirchSapling {
    fn to_id(&self) -> i32 {
        return 10432;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10432 {
            return Some(PottedBirchSapling {
            });
        }
        return None;
    }
}

