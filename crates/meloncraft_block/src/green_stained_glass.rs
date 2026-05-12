use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GreenStainedGlass {
}


impl BlockState for GreenStainedGlass {
    fn to_id(&self) -> i32 {
        return 6910;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6910 {
            return Some(GreenStainedGlass {
            });
        }
        return None;
    }
}

