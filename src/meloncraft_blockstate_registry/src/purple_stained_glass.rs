use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PurpleStainedGlass {
}


impl BlockState for PurpleStainedGlass {
    fn to_id(&self) -> i32 {
        return 6907;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 6907 {
            return Some(PurpleStainedGlass {
            });
        }
        return None;
    }
}

