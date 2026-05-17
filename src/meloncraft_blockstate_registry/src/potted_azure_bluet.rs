use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PottedAzureBluet {
}


impl BlockState for PottedAzureBluet {
    fn to_id(&self) -> i32 {
        return 10444;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 10444 {
            return Some(PottedAzureBluet {
            });
        }
        return None;
    }
}

