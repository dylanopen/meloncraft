use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AzureBluet {
}


impl BlockState for AzureBluet {
    fn to_id(&self) -> i32 {
        return 2126;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2126 {
            return Some(AzureBluet {
            });
        }
        return None;
    }
}

