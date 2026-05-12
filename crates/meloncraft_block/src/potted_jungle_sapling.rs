use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedJungleSapling {
}


impl BlockState for PottedJungleSapling {
    fn to_id(&self) -> i32 {
        return 10433;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10433 {
            return Some(PottedJungleSapling {
            });
        }
        return None;
    }
}

