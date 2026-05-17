use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortDryGrass {
}


impl BlockState for ShortDryGrass {
    fn to_id(&self) -> i32 {
        return 2052;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 2052 {
            return Some(ShortDryGrass {
            });
        }
        return None;
    }
}

