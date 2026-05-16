use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriedKelpBlock {
}


impl BlockState for DriedKelpBlock {
    fn to_id(&self) -> i32 {
        return 14887;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14887 {
            return Some(DriedKelpBlock {
            });
        }
        return None;
    }
}

