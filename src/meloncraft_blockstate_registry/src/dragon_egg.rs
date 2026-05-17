use crate::BlockState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DragonEgg {
}


impl BlockState for DragonEgg {
    fn to_id(&self) -> i32 {
        return 9277;
        panic!("Invalid block state")
    }

    fn from_id(state_id: i32) -> Option<Self> {
        if state_id == 9277 {
            return Some(DragonEgg {
            });
        }
        return None;
    }
}

