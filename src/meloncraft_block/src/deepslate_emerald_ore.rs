use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeepslateEmeraldOre {
}


impl BlockState for DeepslateEmeraldOre {
    fn to_id(&self) -> i32 {
        return 9373;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9373 {
            return Some(DeepslateEmeraldOre {
            });
        }
        return None;
    }
}

