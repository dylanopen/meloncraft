use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HornCoralBlock {
}


impl BlockState for HornCoralBlock {
    fn to_id(&self) -> i32 {
        return 14944;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 14944 {
            return Some(HornCoralBlock {
            });
        }
        return None;
    }
}

