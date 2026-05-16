use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeepslateCoalOre {
}


impl BlockState for DeepslateCoalOre {
    fn to_id(&self) -> i32 {
        return 134;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 134 {
            return Some(DeepslateCoalOre {
            });
        }
        return None;
    }
}

