use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolishedDiorite {
}


impl BlockState for PolishedDiorite {
    fn to_id(&self) -> i32 {
        return 5;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 5 {
            return Some(PolishedDiorite {
            });
        }
        return None;
    }
}

